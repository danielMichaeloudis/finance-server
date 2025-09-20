use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
};

use crate::{
    api::*,
    models::Transaction,
    utils::{get_jwt_provider, get_store, JWTKeyProvider, Store},
    AppState,
};

pub struct ApiBridge {
    state_jwt: State<JWTKeyProvider>,
    state_store: State<Store>,
}

impl ApiBridge {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let jwt_key_provider = get_jwt_provider();
        let store = get_store().await?;
        Ok(Self {
            state_jwt: State::<JWTKeyProvider>(jwt_key_provider),
            state_store: State::<Store>(store),
        })
    }

    pub async fn test_token(self, token: &str) -> Result<(), StatusCode> {
        let header_map = Self::get_header_map(token);
        route_test_token(self.state_jwt, header_map).await
    }

    pub async fn get_transactions(
        self,
        token: &str,
    ) -> Result<Vec<Transaction>, (StatusCode, String)> {
        let header_map = Self::get_header_map(token);
        let t = route_get_all_transactions(self.state_store, self.state_jwt, header_map).await?;
        Ok(t.0)
    }

    fn get_header_map(token: &str) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.append(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
        header_map
    }
}
