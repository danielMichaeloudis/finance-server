pub use auth_routes::*;
use axum::{
    routing::{get, post},
    Router,
};
mod auth_routes;
mod data_routes;
mod user_routes;
pub use data_routes::*;
pub use user_routes::*;

use crate::AppState;

pub(crate) fn api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/transactions",
            get(route_get_all_transactions).post(route_add_transaction),
        )
        .route("/signup", post(route_signup))
        .route("/login", post(route_login))
        .route("/has_family", get(route_get_has_family))
        .route("/test_token", get(route_test_token))
        .route("/vendors", get(route_get_vendors_data))
        .route("/total_spent", get(route_get_total_spent))
        .route("/total_in_out", get(route_get_total_in_out))
        .route("/join_family", post(route_join_family))
        .route("/create_family", get(route_create_family))
        .route("/get_family_join_code", get(route_get_family_join_code))
        .route("/goals", get(route_get_goals).post(route_set_goal))
        .route("/transactions_many", post(route_add_many_transactions))
        .route("/export/{file_type}", get(route_export))
}
