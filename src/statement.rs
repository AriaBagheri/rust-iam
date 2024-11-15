use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::effect::Effect;
use crate::resource::ResourceAbstract;
use crate::traits::Matches;

/// Represents a statement in an IAM policy, defining access control for specific
/// actions and resources, with an associated effect (`Allow` or `Deny`).
///
/// # Type Parameters
/// - `Action`: The type representing an action in the statement.
/// - `Partition`: The type representing the partition of a resource.
/// - `Service`: The type representing the service of a resource.
/// - `Region`: The type representing the region of a resource.
/// - `AccountID`: The type representing the account ID of a resource.
/// - `ResourceType`: The type representing the type of a resource.
/// - `ResourceID`: The type representing the ID of a resource.
///
/// # Fields
/// - `effect`: The `Effect` (either `Allow` or `Deny`) for the statement.
/// - `actions`: A list of actions that this statement applies to.
/// - `resources`: A list of resources that this statement applies to.
///
/// # Examples
///
/// Creating a statement:
/// ```
/// use rust_iam::Statement;
/// use rust_iam::Effect;
/// use rust_iam::ResourceAbstract;
///
/// let statement = Statement {
///     effect: Effect::Allow,
///     actions: vec!["read"],
///     resources: vec![],
/// };
/// assert_eq!(statement.effect, Effect::Allow);
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Statement<
    Action,
    Partition,
    Service,
    Region,
    AccountID,
    ResourceType,
    ResourceID,
> {
    /// The effect of the statement (either `Allow` or `Deny`).
    pub effect: Effect,

    /// A list of actions that this statement applies to.
    pub actions: Vec<Action>,

    /// A list of resources that this statement applies to.
    pub resources: Vec<ResourceAbstract<Partition, Service, Region, AccountID, ResourceType, ResourceID>>,
}

/// Represents the result of evaluating a statement for a given action and resource.
///
/// - `Allow`: Access is explicitly allowed.
/// - `Deny`: Access is explicitly denied.
/// - `NotSpecified`: No matching rules were found in the statement.
#[derive(Debug, PartialEq)]
pub enum MaybeEffect {
    /// Explicitly allows access.
    Allow,

    /// Explicitly denies access.
    Deny,

    /// No matching rules found.
    NotSpecified,
}

impl<
    Action: Matches<bool>,
    Partition: Matches<bool>,
    Service: Matches<bool>,
    Region: Matches<bool>,
    AccountID: Matches<bool>,
    ResourceType: Matches<bool>,
    ResourceID: Matches<bool>,
> Statement<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID>
{
    /// Checks if the given `action` and `resource` match this statement.
    ///
    /// # Arguments
    /// - `action`: The action to evaluate against the statement.
    /// - `resource`: The resource to evaluate against the statement.
    ///
    /// # Returns
    /// - `MaybeEffect::Allow` if the action and resource match and the effect is `Allow`.
    /// - `MaybeEffect::Deny` if the action and resource match and the effect is `Deny`.
    /// - `MaybeEffect::NotSpecified` if no matches are found.
    pub fn matches(
        &self,
        action: &Action,
        resource: &ResourceAbstract<Partition, Service, Region, AccountID, ResourceType, ResourceID>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::Effect;
    use crate::resource::ResourceAbstract;
    use crate::MaybeEffect;

    #[derive(Debug, PartialEq, Clone)]
    struct MockAction(String);

    impl Matches<bool> for MockAction {
        fn matches(&self, other: &Self) -> Result<bool, &'static str> {
            Ok(self.0 == other.0)
        }
    }

    #[test]
    fn test_matches_allow() {
        let statement = Statement {
            effect: Effect::Allow,
            actions: vec![MockAction("read".to_string())],
            resources: vec![ResourceAbstract {
                partition: None::<usize>,
                service: None::<usize>,
                region: None::<usize>,
                account_id: None::<usize>,
                resource_type: None::<usize>,
                resource_id: None,
            }],
        };

        let action = MockAction("read".to_string());
        let resource = ResourceAbstract {
            partition: None,
            service: None,
            region: None,
            account_id: None,
            resource_type: None,
            resource_id: Some("resource_1".to_string()),
        };

        assert_eq!(statement.matches(&action, &resource), MaybeEffect::Allow);
    }

    #[test]
    fn test_matches_deny() {
        let statement = Statement {
            effect: Effect::Deny,
            actions: vec![MockAction("write".to_string())],
            resources: vec![ResourceAbstract {
                partition: None::<usize>,
                service: None::<usize>,
                region: None::<usize>,
                account_id: None::<usize>,
                resource_type: None::<usize>,
                resource_id: Some("resource_2".to_string()),
            }],
        };

        let action = MockAction("write".to_string());
        let resource = ResourceAbstract {
            partition: None,
            service: None,
            region: None,
            account_id: None,
            resource_type: None,
            resource_id: Some("resource_2".to_string()),
        };

        assert_eq!(statement.matches(&action, &resource), MaybeEffect::Deny);
    }

    #[test]
    fn test_matches_not_specified() {
        let statement = Statement {
            effect: Effect::Allow,
            actions: vec![MockAction("read".to_string())],
            resources: vec![ResourceAbstract {
                partition: None::<usize>,
                service: None::<usize>,
                region: None::<usize>,
                account_id: None::<usize>,
                resource_type: None::<usize>,
                resource_id: Some("resource_1".to_string()),
            }],
        };

        let action = MockAction("delete".to_string());
        let resource = ResourceAbstract {
            partition: None::<usize>,
            service: None::<usize>,
            region: None::<usize>,
            account_id: None::<usize>,
            resource_type: None::<usize>,
            resource_id: Some("resource_2".to_string()),
        };

        assert_eq!(statement.matches(&action, &resource), MaybeEffect::NotSpecified);
    }
}