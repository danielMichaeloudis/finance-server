use axum::extract::Query;
use css_helper::Css;
use maud::{html, Markup};

use crate::{
    models::Transaction,
    website::{components::filter_section, pages::FilterParams},
};

pub fn home_page(transaction_list: &[Transaction], query_params: &Query<FilterParams>) -> Markup {
    html! {
        (filter_section(query_params))
    }
}
