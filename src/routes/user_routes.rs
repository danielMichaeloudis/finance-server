use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::utils::{
    create_family, get_family_join_code, get_uuid_from_token, join_family, JWTKeyProvider, Store,
};

pub async fn route_join_family(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(family_join_code): Json<String>,
) -> Result<(), (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    join_family(&user_uuid, family_join_code, &store).await?;
    Ok(())
}

pub async fn route_create_family(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<String>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    let res = create_family(&user_uuid, &store).await?;
    Ok(Json(res))
}

pub async fn route_get_family_join_code(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<Option<String>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    let res = get_family_join_code(&user_uuid, &store).await?;
    Ok(Json(res))
}
