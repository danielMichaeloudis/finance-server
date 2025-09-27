use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    response::AppendHeaders,
    Json,
};
use serde::Serialize;
use serde_json::{json, Value};
use tokio_util::io::ReaderStream;

use crate::{
    models::{Goal, Transaction, VendorData},
    utils::{
        encrypt_add_transactions, encrypt_and_add_transaction, get_all_transactions, get_goals,
        get_total_in_out, get_total_spent, get_uuid_from_token, process_vendor_data, set_goal,
        JWTKeyProvider, Store,
    },
};

pub async fn route_add_transaction(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(transaction): Json<Transaction>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    encrypt_and_add_transaction(&store, &user_uuid, transaction).await?;
    Ok(Json(get_all_transactions(&store, &user_uuid).await?))
}

pub async fn route_add_many_transactions(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
    Json(transactions): Json<Vec<Transaction>>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    encrypt_add_transactions(&store, &user_uuid, transactions).await?;
    Ok(Json(get_all_transactions(&store, &user_uuid).await?))
}

pub async fn route_get_all_transactions(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(get_all_transactions(&store, &user_uuid).await?))
}

pub async fn route_get_vendors_data(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<Vec<VendorData>>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(process_vendor_data(&store, &user_uuid).await?))
}

pub async fn route_get_total_in_out(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(get_total_in_out(&store, &user_uuid).await?))
}

pub async fn route_get_total_spent(
    State(store): State<Store>,
    State(jwt_key_provider): State<JWTKeyProvider>,
    header_map: HeaderMap,
) -> Result<Json<f64>, (StatusCode, String)> {
    let user_uuid = get_uuid_from_token(&jwt_key_provider, &header_map).await?;
    Ok(Json(get_total_spent(&store, &user_uuid).await?))
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
    let transactions = json!(get_all_transactions(&store, &user_uuid).await?).to_string();
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
