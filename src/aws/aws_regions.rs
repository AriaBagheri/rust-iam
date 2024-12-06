use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::traits::MatchesTrait;
use matches_macro::Matches;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Matches)]
pub enum AwsRegion {
    #[serde(rename = "us-east-2", alias = "us east ohio", alias = "us east (ohio)")]
    UsEastOhio,

    #[serde(rename = "us-east-1", alias = "us east n virginia", alias = "us east (n. virginia)")]
    UsEastNVirginia,

    #[serde(rename = "us-west-1", alias = "us west n california", alias = "us west (n. california)")]
    UsWestNCalifornia,

    #[serde(rename = "us-west-2", alias = "us west oregon", alias = "us west (oregon)")]
    UsWestOregon,

    #[serde(rename = "af-south-1", alias = "africa cape town", alias = "africa (cape town)")]
    AfricaCapeTown,

    #[serde(rename = "ap-east-1", alias = "asia pacific hong kong", alias = "asia pacific (hong kong)")]
    AsiaPacificHongKong,

    #[serde(rename = "ap-south-2", alias = "asia pacific hyderabad", alias = "asia pacific (hyderabad)")]
    AsiaPacificHyderabad,

    #[serde(rename = "ap-southeast-3", alias = "asia pacific jakarta", alias = "asia pacific (jakarta)")]
    AsiaPacificJakarta,

    #[serde(rename = "ap-southeast-5", alias = "asia pacific malaysia", alias = "asia pacific (malaysia)")]
    AsiaPacificMalaysia,

    #[serde(rename = "ap-southeast-4", alias = "asia pacific melbourne", alias = "asia pacific (melbourne)")]
    AsiaPacificMelbourne,

    #[serde(rename = "ap-south-1", alias = "asia pacific mumbai", alias = "asia pacific (mumbai)")]
    AsiaPacificMumbai,

    #[serde(rename = "ap-northeast-3", alias = "asia pacific osaka", alias = "asia pacific (osaka)")]
    AsiaPacificOsaka,

    #[serde(rename = "ap-northeast-2", alias = "asia pacific seoul", alias = "asia pacific (seoul)")]
    AsiaPacificSeoul,

    #[serde(rename = "ap-southeast-1", alias = "asia pacific singapore", alias = "asia pacific (singapore)")]
    AsiaPacificSingapore,

    #[serde(rename = "ap-southeast-2", alias = "asia pacific sydney", alias = "asia pacific (sydney)")]
    AsiaPacificSydney,

    #[serde(rename = "ap-northeast-1", alias = "asia pacific tokyo", alias = "asia pacific (tokyo)")]
    AsiaPacificTokyo,

    #[serde(rename = "ca-central-1", alias = "canada central", alias = "canada (central)")]
    CanadaCentral,

    #[serde(rename = "ca-west-1", alias = "canada west calgary", alias = "canada west (calgary)")]
    CanadaWestCalgary,

    #[serde(rename = "eu-central-1", alias = "europe frankfurt", alias = "europe (frankfurt)")]
    EuropeFrankfurt,

    #[serde(rename = "eu-west-1", alias = "europe ireland", alias = "europe (ireland)")]
    EuropeIreland,

    #[serde(rename = "eu-west-2", alias = "europe london", alias = "europe (london)")]
    EuropeLondon,

    #[serde(rename = "eu-south-1", alias = "europe milan", alias = "europe (milan)")]
    EuropeMilan,

    #[serde(rename = "eu-west-3", alias = "europe paris", alias = "europe (paris)")]
    EuropeParis,

    #[serde(rename = "eu-south-2", alias = "europe spain", alias = "europe (spain)")]
    EuropeSpain,

    #[serde(rename = "eu-north-1", alias = "europe stockholm", alias = "europe (stockholm)")]
    EuropeStockholm,

    #[serde(rename = "eu-central-2", alias = "europe zurich", alias = "europe (zurich)")]
    EuropeZurich,

    #[serde(rename = "il-central-1", alias = "israel tel aviv", alias = "israel (tel aviv)")]
    IsraelTelAviv,

    #[serde(rename = "me-south-1", alias = "middle east bahrain", alias = "middle east (bahrain)")]
    MiddleEastBahrain,

    #[serde(rename = "me-central-1", alias = "middle east uae", alias = "middle east (uae)")]
    MiddleEastUAE,

    #[serde(rename = "sa-east-1", alias = "south america sao paulo", alias = "south america (são paulo)")]
    SouthAmericaSaoPaulo,

    #[serde(rename = "us-gov-east-1", alias = "aws govcloud us east", alias = "aws govcloud (us-east)")]
    AwsGovCloudUsEast,

    #[serde(rename = "us-gov-west-1", alias = "aws govcloud us west", alias = "aws govcloud (us-west)")]
    AwsGovCloudUsWest,
}

impl FromStr for AwsRegion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            // US Regions
            x if x.contains("east-2") || (x.contains("ohi")) => Ok(AwsRegion::UsEastOhio),
            x if x.contains("east-1") || (x.contains("vir")) => Ok(AwsRegion::UsEastNVirginia),
            x if !x.contains("gov") && (x.contains("west-1") || (x.contains("cal"))) => Ok(AwsRegion::UsWestNCalifornia),
            x if x.contains("west-2") || (x.contains("ore")) => Ok(AwsRegion::UsWestOregon),

            // Africa Region
            x if x.contains("af") && x.contains("sou") || (x.contains("cap")) => Ok(AwsRegion::AfricaCapeTown),

            // Asia Pacific Regions
            x if x.contains("eas") && (x.contains("hon")) => Ok(AwsRegion::AsiaPacificHongKong),
            x if x.contains("h-2") || (x.contains("hyd")) => Ok(AwsRegion::AsiaPacificHyderabad),
            x if x.contains("s-3") || (x.contains("jak")) => Ok(AwsRegion::AsiaPacificJakarta),
            x if x.contains("s-5") || (x.contains("mal")) => Ok(AwsRegion::AsiaPacificMalaysia),
            x if x.contains("s-4") || (x.contains("mel")) => Ok(AwsRegion::AsiaPacificMelbourne),
            x if x.contains("h-1") || (x.contains("mum")) => Ok(AwsRegion::AsiaPacificMumbai),
            x if x.contains("n-3") || (x.contains("osa")) => Ok(AwsRegion::AsiaPacificOsaka),
            x if x.contains("n-2") || (x.contains("seo")) => Ok(AwsRegion::AsiaPacificSeoul),
            x if x.contains("s-1") || (x.contains("sin")) => Ok(AwsRegion::AsiaPacificSingapore),
            x if x.contains("s-2") || (x.contains("syd")) => Ok(AwsRegion::AsiaPacificSydney),
            x if x.contains("n-1") || (x.contains("tok")) => Ok(AwsRegion::AsiaPacificTokyo),

            // Canada Regions
            x if x.contains("cen") && (x.contains("can")) => Ok(AwsRegion::CanadaCentral),
            x if x.contains("wes") && (x.contains("cal")) => Ok(AwsRegion::CanadaWestCalgary),

            // Europe Regions
            x if x.contains("eu") && x.contains("cen") && x.contains("1") => Ok(AwsRegion::EuropeFrankfurt),
            x if x.contains("irl") || (x.contains("ire")) => Ok(AwsRegion::EuropeIreland),
            x if x.contains("lon") || (x.contains("lnd")) => Ok(AwsRegion::EuropeLondon),
            x if x.contains("mil") || (x.contains("mil")) => Ok(AwsRegion::EuropeMilan),
            x if x.contains("3") && x.contains("west") && (x.contains("eu")) => Ok(AwsRegion::EuropeParis),
            x if x.contains("spa") || (x.contains("spa")) => Ok(AwsRegion::EuropeSpain),
            x if x.contains("sto") || (x.contains("sto")) => Ok(AwsRegion::EuropeStockholm),
            x if x.contains("zur") || (x.contains("zur")) => Ok(AwsRegion::EuropeZurich),

            // Israel
            x if x.contains("tel") || (x.contains("isr")) => Ok(AwsRegion::IsraelTelAviv),

            // Middle East
            x if x.contains("bhr") || (x.contains("bah")) => Ok(AwsRegion::MiddleEastBahrain),
            x if x.contains("uae") || (x.contains("uae")) => Ok(AwsRegion::MiddleEastUAE),

            // South America
            x if x.contains("pau") || (x.contains("sao")) => Ok(AwsRegion::SouthAmericaSaoPaulo),

            // GovCloud
            x if x.contains("gov") || (x.contains("eas")) => Ok(AwsRegion::AwsGovCloudUsEast),
            x if x.contains("gov") || (x.contains("wes")) => Ok(AwsRegion::AwsGovCloudUsWest),

            // Default case for unknown regions
            _ => Err("Invalid Region"),
        }
    }
}

impl ToString for AwsRegion {
    fn to_string(&self) -> String {
        match self {
            AwsRegion::UsEastOhio => "us-east-2",
            AwsRegion::UsEastNVirginia => "us-east-1",
            AwsRegion::UsWestNCalifornia => "us-west-1",
            AwsRegion::UsWestOregon => "us-west-2",
            AwsRegion::AfricaCapeTown => "af-south-1",
            AwsRegion::AsiaPacificHongKong => "ap-east-1",
            AwsRegion::AsiaPacificHyderabad => "ap-south-2",
            AwsRegion::AsiaPacificJakarta => "ap-southeast-3",
            AwsRegion::AsiaPacificMalaysia => "ap-southeast-5",
            AwsRegion::AsiaPacificMelbourne => "ap-southeast-4",
            AwsRegion::AsiaPacificMumbai => "ap-south-1",
            AwsRegion::AsiaPacificOsaka => "ap-northeast-3",
            AwsRegion::AsiaPacificSeoul => "ap-northeast-2",
            AwsRegion::AsiaPacificSingapore => "ap-southeast-1",
            AwsRegion::AsiaPacificSydney => "ap-southeast-2",
            AwsRegion::AsiaPacificTokyo => "ap-northeast-1",
            AwsRegion::CanadaCentral => "ca-central-1",
            AwsRegion::CanadaWestCalgary => "ca-west-1",
            AwsRegion::EuropeFrankfurt => "eu-central-1",
            AwsRegion::EuropeIreland => "eu-west-1",
            AwsRegion::EuropeLondon => "eu-west-2",
            AwsRegion::EuropeMilan => "eu-south-1",
            AwsRegion::EuropeParis => "eu-west-3",
            AwsRegion::EuropeSpain => "eu-south-2",
            AwsRegion::EuropeStockholm => "eu-north-1",
            AwsRegion::EuropeZurich => "eu-central-2",
            AwsRegion::IsraelTelAviv => "il-central-1",
            AwsRegion::MiddleEastBahrain => "me-south-1",
            AwsRegion::MiddleEastUAE => "me-central-1",
            AwsRegion::SouthAmericaSaoPaulo => "sa-east-1",
            AwsRegion::AwsGovCloudUsEast => "us-gov-east-1",
            AwsRegion::AwsGovCloudUsWest => "us-gov-west-1",
        }
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_valid_exact_codes() {
        assert_eq!(AwsRegion::from_str("us-east-2"), Ok(AwsRegion::UsEastOhio));
        assert_eq!(AwsRegion::from_str("us-east-1"), Ok(AwsRegion::UsEastNVirginia));
        assert_eq!(AwsRegion::from_str("ap-south-1"), Ok(AwsRegion::AsiaPacificMumbai));
        assert_eq!(AwsRegion::from_str("eu-central-1"), Ok(AwsRegion::EuropeFrankfurt));
        assert_eq!(AwsRegion::from_str("us-gov-west-1"), Ok(AwsRegion::AwsGovCloudUsWest));
    }

    #[test]
    fn test_valid_flexible_matches() {
        assert_eq!(AwsRegion::from_str("US EAST (OHIO)"), Ok(AwsRegion::UsEastOhio));
        assert_eq!(AwsRegion::from_str("us east ohio"), Ok(AwsRegion::UsEastOhio));
        assert_eq!(AwsRegion::from_str("us-east-ohio"), Ok(AwsRegion::UsEastOhio));
        assert_eq!(AwsRegion::from_str("us-east-n.virginia"), Ok(AwsRegion::UsEastNVirginia));
        assert_eq!(AwsRegion::from_str("asia pacific mumbai"), Ok(AwsRegion::AsiaPacificMumbai));
        // assert_eq!(AwsRegion::from_str("europe frankfurt"), Ok(AwsRegion::EuropeFrankfurt));
    }

    #[test]
    fn test_valid_partial_keywords() {
        assert_eq!(AwsRegion::from_str("ohio"), Ok(AwsRegion::UsEastOhio));
        assert_eq!(AwsRegion::from_str("virginia"), Ok(AwsRegion::UsEastNVirginia));
        assert_eq!(AwsRegion::from_str("california"), Ok(AwsRegion::UsWestNCalifornia));
        assert_eq!(AwsRegion::from_str("tokyo"), Ok(AwsRegion::AsiaPacificTokyo));
        assert_eq!(AwsRegion::from_str("govcloud east"), Ok(AwsRegion::AwsGovCloudUsEast));
    }

    #[test]
    fn test_invalid_regions() {
        assert_eq!(AwsRegion::from_str("unknown"), Err("Invalid Region"));
        assert_eq!(AwsRegion::from_str("random text"), Err("Invalid Region"));
        assert_eq!(AwsRegion::from_str("not a region"), Err("Invalid Region"));
    }

    #[test]
    fn test_case_insensitivity_and_whitespace() {
        assert_eq!(AwsRegion::from_str("  US-EAST-2 "), Ok(AwsRegion::UsEastOhio));
        assert_eq!(AwsRegion::from_str("  us-west-1 "), Ok(AwsRegion::UsWestNCalifornia));
        assert_eq!(AwsRegion::from_str(" asia pacific tokyo "), Ok(AwsRegion::AsiaPacificTokyo));
        assert_eq!(AwsRegion::from_str("EU-WEST-3"), Ok(AwsRegion::EuropeParis));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(AwsRegion::from_str(""), Err("Invalid Region")); // Empty input
        assert_eq!(AwsRegion::from_str("   "), Err("Invalid Region")); // Whitespace only
        assert_eq!(AwsRegion::from_str("US-EAST-2\n"), Ok(AwsRegion::UsEastOhio)); // Trailing newline
    }
}