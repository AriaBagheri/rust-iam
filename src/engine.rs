use std::fmt::Debug;
use std::str::FromStr;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::traits::MatchesTrait;

/// A trait that defines the core types and constraints for an engine-based system.
///
/// The `EngineTrait` is designed to establish a standard interface for defining
/// key components in an engine, such as actions, partitions, services, regions,
/// accounts, resource types, and resource IDs. Each associated type comes with
/// a set of constraints to ensure compatibility and functionality across various
/// operations.
///
/// # Associated Types
///
/// - `Action`: Represents the action to be performed (e.g., "read", "write").
/// - `Partition`: Represents a partition in the system (e.g., "aws", "azure").
/// - `Service`: Represents a service type (e.g., "S3", "EC2").
/// - `Region`: Represents a geographical region (e.g., "us-east-1").
/// - `AccountID`: Represents the account identifier (e.g., user or organizational ID).
/// - `ResourceType`: Represents the type of resource (e.g., "bucket", "instance").
/// - `ResourceID`: Represents the unique identifier of the resource.
///
/// # Constraints
///
/// Each associated type must satisfy the following trait bounds:
///
/// - `Debug`: Ensures the type can be formatted for debugging purposes.
/// - `MatchesTrait<bool>`: Provides a mechanism for custom matching logic.
/// - `Serialize`: Enables the type to be serialized for storage or transmission.
/// - `DeserializeOwned`: Allows the type to be deserialized independently.
/// - `FromStr<Err = &'static str>`: Allows the type to be parsed from a string representation.
/// - `PartialEq`: Ensures equality comparisons can be performed.
/// - `Clone`: Allows duplication of the value.
///```
pub trait EngineTrait: Debug + Default + Copy + Serialize + DeserializeOwned + Sync + Send + Clone + 'static {
    /// The type representing an action within the engine.
    type Action: Debug + MatchesTrait<bool> + Serialize + DeserializeOwned + FromStr<Err=&'static str> + ToString + PartialEq + Eq + Clone + Sync + Send + Clone + 'static;

    /// The type representing a partition (e.g., a system or namespace).
    type Partition: Debug + MatchesTrait<bool> + Serialize + DeserializeOwned + FromStr<Err=&'static str> + ToString + PartialEq + Eq + Clone + Sync + Send + Clone + 'static;

    /// The type representing a service provided by the system.
    type Service: Debug + MatchesTrait<bool> + Serialize + DeserializeOwned + FromStr<Err=&'static str> + ToString + PartialEq + Eq + Clone + Sync + Send + Clone + 'static;

    /// The type representing a geographical region.
    type Region: Debug + MatchesTrait<bool> + Serialize + DeserializeOwned + FromStr<Err=&'static str> + ToString + PartialEq + Eq + Clone + Sync + Send + Clone + 'static;

    /// The type representing an account or user identifier.
    type AccountID: Debug + MatchesTrait<bool> + Serialize + DeserializeOwned + FromStr<Err=&'static str> + ToString + PartialEq + Eq + Clone + Sync + Send + Clone + 'static;

    /// The type representing the resource type (e.g., "bucket", "instance").
    type ResourceType: Debug + MatchesTrait<bool> + Serialize + DeserializeOwned + FromStr<Err=&'static str> + ToString + PartialEq + Eq + Clone + Sync + Send + Clone + 'static;

    /// The type representing the unique identifier for a resource.
    type ResourceID: Debug + MatchesTrait<bool> + Serialize + DeserializeOwned + FromStr<Err=&'static str> + ToString + PartialEq + Eq + Clone + Sync + Send + Clone + 'static;
}
