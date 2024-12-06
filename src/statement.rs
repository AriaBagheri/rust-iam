use serde::{Deserialize, Serialize};
use crate::{Effect, EngineTrait, ResourceAbstract};
use crate::traits::MatchesTrait;

/// Represents a statement in an IAM policy, defining access control rules for actions and resources.
///
/// A `Statement` specifies whether a set of actions is `Allow`ed or `Deny`ed for certain resources.
/// Each statement has an `effect` (either `Allow` or `Deny`), a list of actions, and a list of resources.
///
/// This struct is a key component in evaluating access control in policies, as it determines
/// the effect of a specific action on a resource.
///
/// # Type Parameters
/// - `Engine`: A type implementing the `EngineTrait`, which defines the types and behaviors
///   used in actions and resources.
///
/// # Fields
/// - `effect`: Specifies whether the actions in this statement are allowed or denied.
/// - `actions`: A list of actions (e.g., `read`, `write`) to which this statement applies.
/// - `resources`: A list of resources (e.g., a specific bucket or instance) to which this statement applies.
/// ```
#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct Statement<Engine: EngineTrait> {
    /// Specifies whether the statement allows or denies the actions on the resources.
    pub effect: Effect,

    /// The list of actions that this statement applies to.
    pub actions: Vec<Engine::Action>,

    /// The list of resources that this statement applies to.
    pub resources: Vec<ResourceAbstract<Engine>>,
}
#[cfg(feature = "with-sqlx")]
use sqlx::postgres::PgHasArrayType;

#[cfg(feature = "with-sqlx")]
impl<Engine: EngineTrait> PgHasArrayType for Statement<Engine> {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        // This tells sqlx the PostgreSQL type for an array of JSONB
        sqlx::postgres::PgTypeInfo::with_name("_jsonb")
    }
}
#[cfg(feature = "with-sqlx")]
impl<'r, Engine> sqlx::Decode<'r, sqlx::Postgres> for Statement<Engine>
where
    Engine: EngineTrait, // Ensure Engine has a default implementation
{
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<(dyn StdError + Send + Sync + 'static)>> {
        // Decode the column into a Vec<String> and wrap it in PolicyCollection
        let decoded = <serde_json::Value as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        let policy: Statement<Engine> = serde_json::from_value(decoded)
            .map_err(|e| sqlx::Error::Decode(format!("JSON decode error: {}", e).into()))?;
        Ok(policy)
    }
}

#[cfg(feature = "with-sqlx")]
impl<Engine: EngineTrait> sqlx::Type<sqlx::Postgres> for Statement<Engine> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "with-sqlx")]
impl<'q, Engine> sqlx::Encode<'q, sqlx::Postgres> for Statement<Engine>
where
    Engine: EngineTrait,
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        self.encode_by_ref(buf)
    }
}


use serde::de::{Deserializer, Error, MapAccess, StdError, Visitor};
use std::fmt;

impl<'de, Engine: EngineTrait> Deserialize<'de> for Statement<Engine> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatementVisitor<Engine: EngineTrait>(std::marker::PhantomData<Engine>);

        impl<'de, Engine: EngineTrait> Visitor<'de> for StatementVisitor<Engine> {
            type Value = Statement<Engine>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid Statement object with effect, actions, and resources")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut effect = None;
                let mut actions = None;
                let mut resources = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "effect" => effect = Some(map.next_value()?),
                        "actions" => actions = Some(map.next_value()?),
                        "resources" => resources = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(&key, &["effect", "actions", "resources"])),
                    }
                }

                Ok(Statement {
                    effect: effect.ok_or_else(|| Error::missing_field("effect"))?,
                    actions: actions.ok_or_else(|| Error::missing_field("actions"))?,
                    resources: resources.ok_or_else(|| Error::missing_field("resources"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "Statement",
            &["effect", "actions", "resources"],
            StatementVisitor(std::marker::PhantomData),
        )
    }
}

/// Represents the result of evaluating a statement for a given action and resource.
///
/// The `MaybeEffect` is used to indicate whether access is allowed, denied, or not specified
/// in a given statement.
///
/// # Variants
/// - `Allow`: Access is explicitly allowed.
/// - `Deny`: Access is explicitly denied.
/// - `NotSpecified`: No matching rule was found in the statement.
///
/// # Examples
/// ```rust
/// use rust_iam::MaybeEffect;
///
/// let effect = MaybeEffect::Allow;
/// assert_eq!(effect, MaybeEffect::Allow);
/// ```
#[derive(Debug, PartialEq)]
pub enum MaybeEffect {
    /// Explicitly allows access.
    Allow,

    /// Explicitly denies access.
    Deny,

    /// No matching rule was found.
    NotSpecified,
}

impl<Engine: EngineTrait> Statement<Engine> {
    /// Checks whether the given `action` and `resource` match this statement.
    ///
    /// This method evaluates whether a specific action on a resource matches the
    /// conditions defined in the statement. The following rules apply:
    ///
    /// 1. If the `resource` and `action` both match, and the effect is `Deny`,
    ///    the method returns `MaybeEffect::Deny`.
    /// 2. If the `resource` and `action` both match, and the effect is `Allow`,
    ///    the method sets `is_allow` to `true` but continues evaluating other resources/actions.
    /// 3. If no matches are found, the method returns `MaybeEffect::NotSpecified`.
    ///
    /// # Parameters
    /// - `action`: The action to evaluate against the statement.
    /// - `resource`: The resource to evaluate against the statement.
    ///
    /// # Returns
    /// - `MaybeEffect::Allow` if the action and resource match and the effect is `Allow`.
    /// - `MaybeEffect::Deny` if the action and resource match and the effect is `Deny`.
    /// - `MaybeEffect::NotSpecified` if no matches are found.
    ///```
    pub fn matches(
        &self,
        action: &Engine::Action,
        resource: &ResourceAbstract<Engine>,
    ) -> MaybeEffect {
        let mut is_allow = false;
        for r in self.resources.iter() {
            if let Ok(true) = r.matches(resource) {
                for a in self.actions.iter() {
                    if let Ok(true) = a.matches(action) {
                        if self.effect == Effect::Deny {
                            return MaybeEffect::Deny;
                        } else if self.effect == Effect::Allow {
                            is_allow = true;
                        }
                    }
                }
            }
        }
        if is_allow {
            MaybeEffect::Allow
        } else {
            MaybeEffect::NotSpecified
        }
    }
}