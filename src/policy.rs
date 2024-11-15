use serde::{Deserialize, Serialize};
use crate::{MaybeEffect, ResourceAbstract, Statement};
use crate::traits::Matches;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Policy<
    Action,
    Partition,
    Service,
    Region,
    AccountID,
    ResourceType,
    ResourceID,
> {
    /// An optional unique identifier for the policy.
    pub id: Option<String>,

    /// An optional human-readable name for the policy.
    pub name: Option<String>,

    /// A list of statements defining the policy's access control rules.
    pub statements: Vec<Statement<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID>>,
}

impl<
    Action: Matches<bool>,
    Partition: Matches<bool>,
    Service: Matches<bool>,
    Region: Matches<bool>,
    AccountID: Matches<bool>,
    ResourceType: Matches<bool>,
    ResourceID: Matches<bool>,
> Policy<Action, Partition, Service, Region, AccountID, ResourceType, ResourceID> {
    pub(crate) fn matches(&self, action: &Action, resource: &ResourceAbstract<Partition, Service, Region, AccountID, ResourceType, ResourceID>) -> MaybeEffect {
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
