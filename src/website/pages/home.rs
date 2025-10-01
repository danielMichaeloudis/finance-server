use axum::extract::{Query, Request};
use css_helper::Css;
use maud::{html, Markup};

use crate::{
    api_bridge::ApiBridge,
    models::Transaction,
    website::{
        components::{filter_section, filter_transactions},
        get_cookie,
        pages::FilterParams,
    },
};

pub async fn home_page(req: Request) -> Markup {
    let query_params = Query::<FilterParams>::try_from_uri(req.uri()).unwrap();
    let transaction_list = match ApiBridge::new().await {
        Ok(bridge) => {
            let token = match get_cookie(&req, "token") {
                Some(token) => token,
                None => return html! {},
            };
            match bridge.get_transactions(&token).await {
                Ok(t) => t,
                Err(_) => return html! {},
            }
        }
        Err(_) => return html! {},
    };

    let transaction_list: Vec<Transaction> = filter_transactions(&transaction_list, &query_params);

    html! {
        (filter_section(&query_params))
    }
}
