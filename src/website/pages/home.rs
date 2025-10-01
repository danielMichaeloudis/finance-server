use axum::extract::{Query, Request};
use css_helper::Css;
use maud::{html, Markup};

use crate::{
    api_bridge::ApiBridge,
    website::{
        components::{buyers_card, card_css, filter_section, filter_transactions},
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
    let transaction_list = filter_transactions(&transaction_list, &query_params);
    html! {
        (home_css())
        (card_css())
        (filter_section(&query_params))
        div #cards {
            (buyers_card(&transaction_list))
        }
    }
}

fn home_css() -> Css {
    Css::from(
        r#"
    #cards {
        margin: 1rem;
    }
    "#,
    )
}
