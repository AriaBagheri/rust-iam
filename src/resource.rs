use std::fmt::Debug;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::engine::EngineTrait;
use crate::traits::MatchesTrait;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResourceAbstract<Engine: EngineTrait> {
    //The partition in which the resource is located. A partition is a group of AWS Regions. Each AWS account is scoped to one partition.
    pub partition: Option<Engine::Partition>,
    // The service namespace that identifies the AWS product.
    pub service: Option<Engine::Service>,
    // The Region code. E.g. us-east-2
    pub region: Option<Engine::Region>,
    // The ID of the account that owns the resource.
    pub account_id: Option<Engine::AccountID>,
    // E.g. vpc for virtual private cloud (VPC)
    pub resource_type: Option<Engine::ResourceType>,
    // The resource identifier. The name of the resource, the ID of the resource, or a resource path. Some identifiers include a parent resource sub-resource-type/parent-resource/sub-resource) or a qualifier such as a version (resource-type:resource-name:qualifier)
    pub resource_id: Option<Engine::ResourceID>,
}
use serde::ser::{self, Serializer};
use std::fmt;
impl<Engine: EngineTrait> Serialize for ResourceAbstract<Engine> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fn serialize_field<T: ToString>(field: &Option<T>) -> String {
            match field {
                Some(value) => value.to_string(),
                None => "".to_string(),
            }
        }

        // Construct the colon-separated string
        let serialized_string = format!(
            "arn:{}:{}:{}:{}:{}:{}",
            serialize_field(&self.partition),
            serialize_field(&self.service),
            serialize_field(&self.region),
            serialize_field(&self.account_id),
            serialize_field(&self.resource_type),
            serialize_field(&self.resource_id)
        );

        // Serialize the resulting string
        serializer.serialize_str(&serialized_string)
    }
}

use serde::de::{self, Deserializer, Error, MapAccess, Visitor};
impl<'de, Engine: EngineTrait> Deserialize<'de> for ResourceAbstract<Engine> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResourceAbstractVisitor<Engine: EngineTrait>(std::marker::PhantomData<Engine>);

        impl<'de, Engine: EngineTrait> Visitor<'de> for ResourceAbstractVisitor<Engine> {
            type Value = ResourceAbstract<Engine>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string that can be parsed into a ResourceAbstract")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                ResourceAbstract::<Engine>::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(ResourceAbstractVisitor(std::marker::PhantomData))
    }
}

impl<Engine: EngineTrait> FromStr for ResourceAbstract<Engine>
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("arn:") {
            return Err("Invalid resource format: Resource name should start with 'arn:'".to_string());
        }

        let mut split = s.split(':');

        // Skip the "arn" prefix
        split.next();

        fn flip<T, E>(input: Option<Result<T, E>>) -> Result<Option<T>, E> {
            input.map_or(Ok(None), |res| res.map(Some))
        }

        // Parse the components with proper error handling
        let partition = flip(split.next().map(|f| Engine::Partition::from_str(f)))?;
        let service = flip(split.next().map(|f| Engine::Service::from_str(f)))?;
        let region = flip(split.next().map(|f| Engine::Region::from_str(f)))?;
        let account_id = flip(split.next().map(|f| Engine::AccountID::from_str(f)))?;
        let resource_type = flip(split.next().map(|f| Engine::ResourceType::from_str(f)))?;
        let resource_id = flip(split.next().map(|f| Engine::ResourceID::from_str(f)))?;

        let resource = ResourceAbstract {
            partition,
            service,
            region,
            account_id,
            resource_type,
            resource_id,
        };

        Ok(resource)
    }
}

impl<Engine: EngineTrait> MatchesTrait<bool> for ResourceAbstract<Engine> {
    fn matches(&self, other: &ResourceAbstract<Engine>) -> Result<bool, &'static str> {
        match (self.partition.as_ref(), other.partition.as_ref()) {
            (Some(l), Some(r)) => {
                if !l.matches(r)? {
                    return Ok(false);
                }
            }
            _ => {}
        };
        match (self.service.as_ref(), other.service.as_ref()) {
            (Some(l), Some(r)) => {
                if !l.matches(r)? {
                    return Ok(false);
                }
            }
            _ => {}
        }
        match (self.region.as_ref(), other.region.as_ref()) {
            (Some(l), Some(r)) => {
                if !l.matches(r)? {
                    return Ok(false);
                }
            }
            _ => {}
        }
        match (self.account_id.as_ref(), other.account_id.as_ref()) {
            (Some(l), Some(r)) => {
                if !l.matches(r)? {
                    return Ok(false);
                }
            }
            _ => {}
        }
        match (self.resource_type.as_ref(), other.resource_type.as_ref()) {
            (Some(l), Some(r)) => {
                if !l.matches(r)? {
                    return Ok(false);
                }
            }
            _ => {}
        }
        match (self.resource_id.as_ref(), other.resource_id.as_ref()) {
            (Some(l), Some(r)) => {
                if !l.matches(r)? {
                    return Ok(false);
                }
            }
            _ => {}
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {}