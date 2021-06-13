/// Example for the backend to backend implementation
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

fn validate_token(token: &str) {
    // jsonurl = urlopen("https://"+AUTH0_DOMAIN+"/.well-known/jwks.json")
    let domain = std::env::var("AUTH0_DOMAIN").unwrap();
    let well_known_url = format!("https://{}/.well-known/jwks.json", domain);
}

fn main() {}
