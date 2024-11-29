use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

mod aws_partitions;
mod aws_regions;

use crate::traits::MatchesTrait;
use matches_macro::Matches;
use crate::engine::EngineTrait;

pub use aws_regions::*;
pub use aws_partitions::*;

#[derive(Debug, Copy, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AwsEngine{}

#[derive(Debug, PartialEq, Eq, Matches, Serialize, Deserialize, Clone)]
#[wildcard_matching]
pub struct WildString(pub String);

impl Display for WildString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl FromStr for WildString {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WildString(s.to_string()))
    }
}

impl EngineTrait for AwsEngine {
    type Action = WildString;
    type Partition = AwsPartition;
    type Service = WildString;
    type Region = AwsRegion;
    type AccountID = WildString;
    type ResourceType = WildString;
    type ResourceID = WildString;
}
