use axum::{http::HeaderMap, Json};
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    api_bridge::ApiBridge,
    website::{components::transaction::transaction_popup, get_cookie_from_headers},
};

pub async fn edit_transaction(headers: HeaderMap, Json(transaction_uuid): Json<Uuid>) -> Markup {
    let transaction = match ApiBridge::new().await {
        Ok(bridge) => match get_cookie_from_headers(&headers, "token") {
            Some(token) => bridge
                .get_transaction_by_uuid(&token, transaction_uuid)
                .await
                .ok()
                .map(|t| vec![(transaction_uuid, t)]),
            None => None,
        },
        Err(_) => None,
    };
    html! {
        (transaction_popup(Some("Edit Transaction"), Some("/edit_transaction.js"), transaction.as_deref()))
    }
}
