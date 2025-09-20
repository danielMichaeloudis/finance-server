use std::sync::Arc;

use base64::{engine::general_purpose, Engine};
use jwt_simple::prelude::ES256KeyPair;

#[derive(Clone)]
pub(crate) struct JWTKeyProvider {
    pub key_pair: Arc<ES256KeyPair>,
}

impl JWTKeyProvider {
    pub fn new(jwt_base64: &str) -> Self {
        Self {
            key_pair: Arc::new(
                ES256KeyPair::from_bytes(&general_purpose::STANDARD.decode(jwt_base64).unwrap())
                    .unwrap(),
            ),
        }
    }
}

pub fn get_jwt_provider() -> JWTKeyProvider {
    dotenvy::dotenv().ok();
    let jwt_base64 = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    JWTKeyProvider::new(&jwt_base64)
}
