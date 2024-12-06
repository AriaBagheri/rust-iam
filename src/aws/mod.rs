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


#[cfg(feature = "with-sqlx")]
use sqlx::{Decode, Encode, Type, Postgres};
use serde::ser::StdError;


#[cfg(feature = "with-sqlx")]
impl<'r> Decode<'r, Postgres> for WildString {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<(dyn StdError + Send + Sync + 'static)>> {
        // Delegate decoding to String and wrap the result in PasswordHash
        let decoded = <String as Decode<Postgres>>::decode(value)?;
        Ok(WildString(decoded))
    }
}

#[cfg(feature = "with-sqlx")]
impl Type<Postgres> for WildString {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("VARCHAR")
    }
}

#[cfg(feature = "with-sqlx")]
impl Encode<'_, Postgres> for WildString {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        <std::string::String as sqlx::Encode<'_, Postgres>>::encode_by_ref(&self.0, buf)
    }
}

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
