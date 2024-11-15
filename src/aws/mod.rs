use crate::aws::aws_partitions::AwsPartition;
use crate::aws::aws_regions::AwsRegion;
use crate::resource::ResourceAbstract;

mod aws_partitions;
mod aws_regions;

pub use aws_partitions::*;
pub use aws_regions::*;
pub type AwsResource = ResourceAbstract<
    AwsPartition,
    String,
    AwsRegion,
    u64,
    String,
    String,
>;