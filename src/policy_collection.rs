use crate::{MaybeEffect, Policy, ResourceAbstract};
use crate::traits::Matches;

/// A collection of policies that can validate access to resources based on actions.
///
/// # Type Parameters
/// - `Action`: Type of action to be validated (e.g., read, write).
/// - `Partition`: Type representing the resource partition (e.g., AWS, Azure).
/// - `Service`: Type representing the service (e.g., S3, EC2).
/// - `Region`: Type representing the region (e.g., `us-east-1`).
/// - `AccountID`: Type representing the account ID.
/// - `ResourceType`: Type representing the resource type (e.g., bucket, instance).
/// - `ResourceID`: Type representing the resource ID (e.g., a unique identifier).
pub struct PolicyCollection<
    Action, Partition, Service, Region, AccountID, ResourceType, ResourceID
>(Vec<Policy<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID>>);

/// Implements the `Extend` trait for `PolicyCollection`, allowing policies to be added from an iterator.
impl<
    Action, Partition, Service, Region, AccountID, ResourceType, ResourceID
> Extend<Policy<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID>> for PolicyCollection<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID> {
    /// Extends the policy collection with the contents of an iterator.
    fn extend<I: IntoIterator<Item = Policy<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID>>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<
    Action: Matches<bool>,
    Partition: Matches<bool>,
    Service: Matches<bool>,
    Region: Matches<bool>,
    AccountID: Matches<bool>,
    ResourceType: Matches<bool>,
    ResourceID: Matches<bool>,
> PolicyCollection<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID> {
    /// Validates whether the given action is allowed on the specified resource.
    ///
    /// # Parameters
    /// - `action`: The action to validate (e.g., `Read`, `Write`).
    /// - `resource`: The resource against which the action is validated.
    ///
    /// # Returns
    /// - `true` if the action is allowed by at least one policy and no policy explicitly denies it.
    /// - `false` if any policy explicitly denies the action.
    pub fn validate(&self, action: &Action, resource: &ResourceAbstract<Partition, Service, Region, AccountID, ResourceType, ResourceID>) -> bool {
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
