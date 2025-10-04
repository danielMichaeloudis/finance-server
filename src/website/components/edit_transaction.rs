use axum::Json;
use maud::{html, Markup};

use crate::{models::Transaction, website::components::transaction::transaction_popup};

pub fn edit_transaction(Json(transaction): Json<Transaction>) -> Markup {
    html! {
        (transaction_popup(Some("Edit Transaction"), Some("/edit_transaction.js"), Some(&[transaction])))
    }
}
