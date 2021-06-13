use serde::{Deserialize, Serialize};

/// https://auth0.com/docs/tokens/json-web-tokens/json-web-key-set-properties
/// https://auth0.com/blog/navigating-rs256-and-jwks/

/// A JWK struct that follows the spec and implements Serialize/Deserialize
/// See https://datatracker.ietf.org/doc/html/rfc7517#section-4
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct BaseJwk {
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.1
    pub kty: String,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.2
    pub r#use: String,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<Vec<String>>,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.4
    pub alg: String,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.5
    pub kid: String,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5u: Option<String>,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.7
    /// The contents are base64 encoded but NOT base64 url encoded
    pub x5c: Vec<String>,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.8
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
    /// https://datatracker.ietf.org/doc/html/rfc7517#section-4.9
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5t_s256: Option<String>,
}

/// A JWK with the specific element for each algorithm
/// https://datatracker.ietf.org/doc/html/rfc7518
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Jwk {
    EC {
        alg: String,
        crv: String,
        x: String,
        y: String,
        #[serde(flatten)]
        base: BaseJwk,
    },
    RSA {
        alg: String,
        n: String,
        e: String,
        #[serde(flatten)]
        base: BaseJwk,
    },
    #[serde(rename = "oct")]
    Oct {
        alg: String,
        k: String,
        #[serde(flatten)]
        base: BaseJwk,
    },
}

pub type Jwks = Vec<AlgJwk>;
