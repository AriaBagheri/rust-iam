use crate::{MaybeEffect, Policy, ResourceAbstract};
use crate::engine::EngineTrait;

/// A collection of policies that determine access control for resources based on actions.
///
/// The `PolicyCollection` encapsulates a list of policies (`Policy`) and provides functionality
/// for evaluating whether a given action is allowed or denied for a specified resource. It also
/// supports extending the collection with additional policies.
///
/// # Type Parameters
/// - `Engine`: A type implementing the `EngineTrait`, which defines the core types and behaviors
///   used by the policies in the collection.
///
/// # Purpose
/// The `PolicyCollection` is typically used in systems where multiple policies must be evaluated
/// together to decide access control. Each policy in the collection contains a set of statements
/// that define allow or deny rules for actions on resources.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PolicyCollection<Engine: EngineTrait>(pub Vec<Policy<Engine>>);

#[cfg(feature = "with-sqlx")]
impl<'r, Engine> sqlx::Decode<'r, sqlx::Postgres> for PolicyCollection<Engine>
where
    Engine: EngineTrait, // Ensure Engine has a default implementation
{
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<(dyn StdError + Send + Sync + 'static)>> {
        // Decode the column into a Vec<String> and wrap it in PolicyCollection
        let decoded = <Vec<Policy<Engine>> as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(PolicyCollection(decoded))
    }
}

#[cfg(feature = "with-sqlx")]
impl<Engine: EngineTrait> sqlx::Type<sqlx::Postgres> for PolicyCollection<Engine> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <Vec<serde_json::Value> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "with-sqlx")]
impl<'q, Engine> sqlx::Encode<'q, sqlx::Postgres> for PolicyCollection<Engine>
where
    Engine: EngineTrait,
{
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        self.0.encode_by_ref(buf)
    }
}

use serde::de::{Deserialize, Deserializer, Error, SeqAccess, StdError, Visitor};
use std::fmt;

impl<'de, Engine: EngineTrait> Deserialize<'de> for PolicyCollection<Engine> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PolicyCollectionVisitor<Engine: EngineTrait>(std::marker::PhantomData<Engine>);

        impl<'de, Engine: EngineTrait> Visitor<'de> for PolicyCollectionVisitor<Engine> {
            type Value = PolicyCollection<Engine>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a list of policies")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut policies = Vec::new();

                while let Some(policy) = seq.next_element::<Policy<Engine>>()? {
                    policies.push(policy);
                }

                Ok(PolicyCollection(policies))
            }
        }

        deserializer.deserialize_seq(PolicyCollectionVisitor(std::marker::PhantomData))
    }
}

#[cfg(feature = "with-sea-orm")]
impl<Engine: EngineTrait> Into<sea_orm::Value> for PolicyCollection<Engine> {
    fn into(self) -> sea_orm::Value {
        sea_orm::Value::Array(sea_orm::sea_query::ArrayType::Json, Some(Box::new(self.0.into_iter().map(|p| Into::<sea_orm::Value>::into(p)).collect())))
    }
}

#[cfg(feature = "with-sea-orm")]
use sea_orm::TryGetableFromJson;

#[cfg(feature = "with-sea-orm")]
impl<Engine: EngineTrait> TryGetableFromJson for PolicyCollection<Engine> {

}

#[cfg(feature = "with-sea-orm")]
impl<Engine: EngineTrait> sea_orm::sea_query::ValueType for PolicyCollection<Engine> {
    fn try_from(v: sea_orm::sea_query::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        if let sea_orm::sea_query::Value::Array(_, Some(values)) = v {
            let policies: Vec<Policy<Engine>> = values
                .into_iter()
                .map(|val| match val {
                    sea_orm::sea_query::Value::Json(Some(json)) => {
                        serde_json::from_value(json.deref().clone()).map_err(|_| sea_orm::sea_query::ValueTypeErr)
                    }
                    _ => Err(sea_orm::sea_query::ValueTypeErr),
                })
                .collect::<Result<_, _>>()?;
            Ok(PolicyCollection(policies))
        } else {
            Err(sea_orm::sea_query::ValueTypeErr)
        }
    }

    fn type_name() -> String {
        serde_json::Value::type_name()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        serde_json::Value::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        serde_json::Value::column_type()
    }
}

impl<Engine: EngineTrait> Deref for PolicyCollection<Engine> {
    type Target = Vec<Policy<Engine>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::ops::{Deref, DerefMut};
impl<Engine: EngineTrait> DerefMut for PolicyCollection<Engine> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Engine: EngineTrait> Default for PolicyCollection<Engine> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

/// Implements the `Extend` trait for `PolicyCollection`, allowing policies to be added from an iterator.
///
/// This trait enables the `extend` method to be called on a `PolicyCollection`, making it easy
/// to add multiple policies in bulk. The new policies are appended to the existing collection.
/// ```
impl<Engine: EngineTrait> Extend<Policy<Engine>> for PolicyCollection<Engine> {
    /// Extends the policy collection with the contents of an iterator.
    ///
    /// This method appends the policies from the given iterator to the existing collection.
    ///
    /// # Parameters
    /// - `iter`: An iterator yielding `Policy<Engine>` items to be added to the collection.
    /// ```
    fn extend<I: IntoIterator<Item = Policy<Engine>>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<Engine: EngineTrait> PolicyCollection<Engine> {
    /// Validates whether the given action is allowed on the specified resource.
    ///
    /// This method evaluates all policies in the collection to determine the effect (`MaybeEffect`)
    /// of the action on the resource. The following rules are applied:
    ///
    /// 1. If any policy explicitly denies the action, the method returns `false` immediately.
    /// 2. If no denial is found but at least one policy explicitly allows the action, the method
    ///    returns `true`.
    /// 3. If neither allow nor deny is specified by any policy, the method returns `false`.
    ///
    /// # Parameters
    /// - `action`: The action to validate (e.g., `Read`, `Write`).
    /// - `resource`: The resource to validate the action against.
    ///
    /// # Returns
    /// - `true` if the action is allowed and not denied by any policy.
    /// - `false` if the action is explicitly denied or not explicitly allowed.
    /// ```
    pub fn validate(&self, action: &Engine::Action, resource: &ResourceAbstract<Engine>) -> bool {
        let mut is_allowed = false;
        for policy in &self.0 {
            match policy.matches(action, resource) {
                MaybeEffect::Allow => { is_allowed = true }
                MaybeEffect::Deny => { return false }
                MaybeEffect::NotSpecified => {}
            }
        }
        is_allowed
    }
}