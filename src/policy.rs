use serde::{Deserialize, Serialize};
use crate::{MaybeEffect, ResourceAbstract, Statement};
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Policy<Engine: EngineTrait> {
    /// An optional unique identifier for the policy.
    ///
    /// This identifier can be used to reference or manage the policy within
    /// the system. It is not required for the policy's functionality.
    pub id: Option<String>,

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
    ///     id: Some("policy_1".to_string()),
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