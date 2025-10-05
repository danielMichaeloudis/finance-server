use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{Goal, Transaction, VendorData},
    utils::{
        encrypt_add_transaction, encrypt_add_transactions, encrypt_edit_transaction,
        get_all_transactions, get_goals, get_uuid_from_token, internal_server_error,
        process_vendor_data, remove_transaction, set_goal, JWTKeyProvider, Store,
    },
};

pub async fn route_add_transaction(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(transaction): Json<Transaction>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    encrypt_add_transaction(&store, &user_uuid, transaction).await?;
    Ok(Json(
        get_all_transactions(&store, &user_uuid)
            .await?
            .into_values()
            .collect(),
    ))
}

pub async fn route_add_many_transactions(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(transactions): Json<Vec<Transaction>>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    encrypt_add_transactions(&store, &user_uuid, transactions).await?;
    Ok(Json(
        get_all_transactions(&store, &user_uuid)
            .await?
            .into_values()
            .collect(),
    ))
}

pub async fn route_get_all_transactions(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<HashMap<Uuid, Transaction>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(get_all_transactions(&store, &user_uuid).await?))
}

pub async fn route_get_transaction_by_uuid(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Transaction>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(match get_all_transactions(&store, &user_uuid).await {
        Ok(t) => t
            .get(&uuid)
            .ok_or_else(|| internal_server_error("Transaction Does Not Exist"))
            .map(|t| t.to_owned()),
        Err(e) => Err(e),
    }?))
}

pub async fn route_edit_transaction(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(transaction): Json<Transaction>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    encrypt_edit_transaction(&store, &user_uuid, transaction).await?;
    Ok(Json(
        get_all_transactions(&store, &user_uuid)
            .await?
            .into_values()
            .collect(),
    ))
}

pub async fn route_remove_transaction(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(uuid): Json<Uuid>,
) -> Result<(), (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    remove_transaction(&store, &user_uuid, &uuid).await
}

pub async fn route_get_vendors_data(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<Vec<VendorData>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(process_vendor_data(&store, &user_uuid).await?))
}

pub async fn route_set_goal(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(goal): Json<Goal>,
) -> Result<(), (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    set_goal(&store, &user_uuid, goal).await
}

pub async fn route_get_goals(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<Vec<Goal>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(get_goals(&store, &user_uuid).await?))
}

pub async fn route_export(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Response<String>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    let transactions = json!(get_all_transactions(&store, &user_uuid)
        .await?
        .into_values()
        .collect::<Vec<_>>())
    .to_string();
    let mut res = Response::new(transactions);
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
    );
    res.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str("attachment; filename=\"Transactions.json\"").unwrap(),
    );

    Ok(res)
}
