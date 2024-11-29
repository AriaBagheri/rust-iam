use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned, StdError};
use crate::{MaybeEffect, PolicyCollection, ResourceAbstract, Statement};
use crate::engine::EngineTrait;

/// Represents an access control policy within the system.
///
/// A `Policy` is composed of multiple statements that define specific access
/// control rules. Each policy can have an optional unique identifier and a
/// human-readable name. The primary role of a `Policy` is to evaluate whether
/// a given action is permitted on a specific resource based on its statements.
///
/// # Type Parameters
/// - `Engine`: A type implementing the `EngineTrait`, which defines the core
///   types and behaviors used by the policy (e.g., actions, resources).
#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct Policy<Engine: EngineTrait> {
    /// An optional human-readable name for the policy.
    ///
    /// This name is intended for display purposes and can help users
    /// understand the purpose or scope of the policy.
    pub name: Option<String>,

    /// A list of statements defining the policy's access control rules.
    ///
    /// Each statement specifies conditions under which an action is allowed
    /// or denied for specific resources. Policies are evaluated by iterating
    /// through these statements.
    pub statements: Vec<Statement<Engine>>,
}


#[cfg(feature = "with-sqlx")]
impl<'r, Engine> sqlx::Decode<'r, sqlx::Postgres> for Policy<Engine>
where
    Engine: EngineTrait, // Ensure Engine has a default implementation
{
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<(dyn StdError + Send + Sync + 'static)>> {
        // Decode the column into a Vec<String> and wrap it in PolicyCollection
        let decoded = <serde_json::Value as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        let policy: Policy<Engine> = serde_json::from_value(decoded)
            .map_err(|e| sqlx::Error::Decode(format!("JSON decode error: {}", e).into()))?;
        Ok(policy)
    }
}

#[cfg(feature = "with-sqlx")]
impl<Engine: EngineTrait> sqlx::Type<sqlx::Postgres> for Policy<Engine> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <serde_json::Value as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "with-sqlx")]
impl<'q, Engine> sqlx::Encode<'q, sqlx::Postgres> for Policy<Engine>
where
    Engine: EngineTrait,
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        self.encode_by_ref(buf)
    }
}


#[cfg(feature = "with-sea-orm")]
use serde_json::json;

#[cfg(feature = "with-sea-orm")]
impl<Engine: EngineTrait> Into<sea_orm::Value> for Policy<Engine> {
    fn into(self) -> sea_orm::Value {
        sea_orm::Value::Json(Some(Box::new(json!({
            "name": self.name,
            "statements": self.statements,
        }))))
    }
}

impl<Engine: EngineTrait> Policy<Engine> {
    /// Evaluates the policy against a given action and resource.
    ///
    /// This method determines the effect (`MaybeEffect`) of the policy by
    /// iterating through its statements. The following rules are applied:
    ///
    /// 1. If any statement explicitly denies the action, the method returns
    ///    `MaybeEffect::Deny` immediately.
    /// 2. If no deny is encountered but at least one statement allows the
    ///    action, the method returns `MaybeEffect::Allow`.
    /// 3. If neither allow nor deny is specified by the statements, the method
    ///    returns `MaybeEffect::NotSpecified`.
    ///
    /// # Parameters
    /// - `action`: The action to evaluate against the policy.
    /// - `resource`: The resource to evaluate against the policy.
    ///
    /// # Returns
    /// - `MaybeEffect::Allow`: The action is explicitly allowed.
    /// - `MaybeEffect::Deny`: The action is explicitly denied.
    /// - `MaybeEffect::NotSpecified`: No explicit allow or deny was specified.
    ///
    /// # Example
    /// ```rust
    /// use rust_iam::{Policy, ResourceAbstract, MaybeEffect, EngineTrait};
    ///
    /// struct MyEngine;
    ///
    /// impl EngineTrait for MyEngine {
    ///     // Define associated types...
    /// }
    ///
    /// let policy = Policy::<MyEngine> {
    ///     name: Some("Example Policy".to_string()),
    ///     statements: vec![], // Add actual statements here
    /// };
    ///
    /// let action = ...; // Define an action of type MyEngine::Action
    /// let resource = ...; // Define a resource of type ResourceAbstract<MyEngine>
    ///
    /// match policy.matches(&action, &resource) {
    ///     MaybeEffect::Allow => println!("Action is allowed."),
    ///     MaybeEffect::Deny => println!("Action is denied."),
    ///     MaybeEffect::NotSpecified => println!("No specific effect."),
    /// }
    /// ```
    pub fn matches(&self, action: &Engine::Action, resource: &ResourceAbstract<Engine>) -> MaybeEffect {
        let mut is_allowed = false;
        for statement in self.statements.iter() {
            match statement.matches(action, resource) {
                MaybeEffect::Allow => is_allowed = true,
                MaybeEffect::Deny => return MaybeEffect::Deny,
                _ => {}
            }
        }
        if is_allowed {
            MaybeEffect::Allow
        } else {
            MaybeEffect::NotSpecified
        }
    }
}

use serde::de::{Deserializer, Error, MapAccess, Visitor};
use std::fmt;
impl<'de, Engine: EngineTrait + DeserializeOwned> Deserialize<'de> for Policy<Engine> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PolicyVisitor<Engine: EngineTrait + DeserializeOwned>(std::marker::PhantomData<Engine>);

        impl<'de, Engine: EngineTrait + DeserializeOwned> Visitor<'de> for PolicyVisitor<Engine> {
            type Value = Policy<Engine>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid Policy object with id, name, and statements")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut name = None;
                let mut statements = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => name = Some(map.next_value()?),
                        "statements" => statements = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(&key, &["name", "statements"])),
                    }
                }

                Ok(Policy {
                    name,
                    statements: statements.ok_or_else(|| Error::missing_field("statements"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "Policy",
            &["name", "statements"],
            PolicyVisitor(std::marker::PhantomData),
        )
    }
}