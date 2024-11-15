use serde::{Deserialize, Serialize};

/// Represents the effect of a policy decision: either `Allow` or `Deny`.
///
/// This enum is used in contexts where access control or authorization
/// decisions are being made. It is serialized and deserialized to/from
/// strings using Serde with the values `"allow"` and `"deny"`.
///
/// # Examples
///
/// Serialize an `Effect` into JSON:
/// ```
/// use serde_json;
/// use rust_iam::Effect;
///
/// let effect = Effect::Allow;
/// let json = serde_json::to_string(&effect).unwrap();
/// assert_eq!(json, "\"allow\"");
/// ```
///
/// Deserialize an `Effect` from JSON:
/// ```
/// use serde_json;
/// use rust_iam::Effect;
///
/// let json = "\"deny\"";
/// let effect: Effect = serde_json::from_str(json).unwrap();
/// assert_eq!(effect, Effect::Deny);
/// ```
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Effect {
    /// Represents an "allow" policy decision.
    #[serde(rename = "allow")]
    Allow,

    /// Represents a "deny" policy decision.
    #[serde(rename = "deny")]
    Deny,
}