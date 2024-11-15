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
pub struct PolicyCollection<Engine: EngineTrait>(Vec<Policy<Engine>>);

/// Implements the `Extend` trait for `PolicyCollection`, allowing policies to be added from an iterator.
///
/// This trait enables the `extend` method to be called on a `PolicyCollection`, making it easy
/// to add multiple policies in bulk. The new policies are appended to the existing collection.
///
/// # Example
/// ```rust
/// use rust_iam::{Policy, PolicyCollection, EngineTrait};
///
/// struct MyEngine;
/// impl EngineTrait for MyEngine {
///     // Define associated types...
/// }
///
/// let mut policies = PolicyCollection::<MyEngine>(vec![]);
/// let new_policies = vec![Policy::new(), Policy::new()]; // Example policies
/// policies.extend(new_policies);
/// ```
impl<Engine: EngineTrait> Extend<Policy<Engine>> for PolicyCollection<Engine> {
    /// Extends the policy collection with the contents of an iterator.
    ///
    /// This method appends the policies from the given iterator to the existing collection.
    ///
    /// # Parameters
    /// - `iter`: An iterator yielding `Policy<Engine>` items to be added to the collection.
    ///
    /// # Example
    /// ```rust
    /// use rust_iam::PolicyCollection;
    /// let mut collection = PolicyCollection::<MyEngine>(vec![]);
    /// let additional_policies = vec![policy1, policy2];
    /// collection.extend(additional_policies);
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
    ///
    /// # Example
    /// ```rust
    /// use rust_iam::PolicyCollection;
    /// let collection = PolicyCollection::<MyEngine>(vec![
    ///     policy1,
    ///     policy2,
    /// ]);
    /// let action = ...; // Define action
    /// let resource = ...; // Define resource
    ///
    /// let result = collection.validate(&action, &resource);
    /// if result {
    ///     println!("Action is allowed.");
    /// } else {
    ///     println!("Action is denied or not specified.");
    /// }
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