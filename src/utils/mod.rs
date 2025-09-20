mod auth;
mod data_processing;
mod encryption;
mod jwt;
mod state;
mod store;
mod user_processing;

use axum::http::StatusCode;

pub(crate) use auth::get_uuid_from_token;
pub(crate) use data_processing::*;
pub(crate) use encryption::{decrypt_data, encrypt_data};
pub(crate) use jwt::{get_jwt_provider, JWTKeyProvider};
pub(crate) use state::AppState;
pub(crate) use store::{get_store, Store};
pub(crate) use user_processing::*;

pub(crate) fn internal_server_error(e: impl ToString) -> (StatusCode, String) {
    println!("Internal Server Error: {}", e.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
