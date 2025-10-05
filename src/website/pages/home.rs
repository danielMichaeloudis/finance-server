use std::collections::HashMap;

use axum::extract::Query;
use css_helper::Css;
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    models::Transaction,
    website::{components::filter_section, pages::FilterParams},
};

pub fn home_page(
    _transaction_list: &HashMap<Uuid, Transaction>,
    query_params: &Query<FilterParams>,
) -> Markup {
    html! {
        (filter_section(query_params))
    }
}
