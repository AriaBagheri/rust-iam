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
///
/// # Examples
/// ```rust
/// use rust_iam::{Effect, Statement, ResourceAbstract};
///
/// let statement = Statement {
///     effect: Effect::Allow,
///     actions: vec!["read"],
///     resources: vec![],
/// };
/// assert_eq!(statement.effect, Effect::Allow);
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Statement<Engine: EngineTrait> {
    /// Specifies whether the statement allows or denies the actions on the resources.
    pub effect: Effect,

    /// The list of actions that this statement applies to.
    pub actions: Vec<Engine::Action>,

    /// The list of resources that this statement applies to.
    pub resources: Vec<ResourceAbstract<Engine>>,
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
    ///
    /// # Examples
    /// ```rust
    /// use rust_iam::{Effect, MaybeEffect, Statement};
    /// let statement = Statement::<MyEngine> {
    ///     effect: Effect::Allow,
    ///     actions: vec!["read"],
    ///     resources: vec!["resource_1"],
    /// };
    ///
    /// let action = "read";
    /// let resource = "resource_1";
    ///
    /// assert_eq!(statement.matches(&action, &resource), MaybeEffect::Allow);
    /// ```
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