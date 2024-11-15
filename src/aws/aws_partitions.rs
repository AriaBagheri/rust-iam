use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::traits::MatchesTrait;
use matches_macro::Matches;

#[derive(Debug, Serialize, Deserialize, PartialEq, Matches, Clone)]
pub enum AwsPartition {
    #[serde(rename="aws")]
    Aws,
    #[serde(rename="aws-cn")]
    AwsChina,
    #[serde(rename="aws-us-gov")]
    AwsUsGov
}

impl FromStr for AwsPartition {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            x if x.contains("ch") => Ok(AwsPartition::AwsChina),
            x if x.contains("us") => Ok(AwsPartition::AwsUsGov),
            x if x.contains("aws") => Ok(AwsPartition::AwsChina),
            _ => Err("no match"),
        }
    }
}
