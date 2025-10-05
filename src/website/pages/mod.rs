mod home;
mod login;
mod signup;
mod table;

use std::collections::HashMap;

use axum::extract::{Query, Request};
use chrono::NaiveDate;
pub use home::home_page;
pub use login::login_page;
use maud::{html, Markup, DOCTYPE};
use serde::{Deserialize, Serialize};
pub use signup::signup_page;
pub use table::table_page;
use uuid::Uuid;

use crate::{
    api_bridge::ApiBridge,
    models::Transaction,
    website::{
        components::{filter_transactions, header, navigation_bar},
        get_cookie,
    },
};
use css_helper::{Colour, Css, Theme, ThemeValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterParams {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub tags: Option<String>,
    pub buyer: Option<String>,
    pub item_bought_for: Option<String>,
}

pub async fn authorised_page(
    content: impl Fn(&HashMap<Uuid, Transaction>, &Query<FilterParams>) -> Markup,
    req: Request,
) -> Markup {
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

    let transaction_list: HashMap<Uuid, Transaction> =
        filter_transactions(transaction_list, &query_params);

    let total_in = transaction_list
        .iter()
        .fold(0.0, |acc, (_, t)| acc + 0.0f64.max(t.cost));
    let total_out = match transaction_list
        .iter()
        .fold(0.0, |acc, (_, t)| acc + -0.0f64.min(t.cost))
    {
        0.0 => 0.0,
        t => -t,
    };
    let total = match total_in - total_out {
        0.0 => 0.0,
        t => -t,
    };

    let wrapped = html! {
        (authorised_page_css())
        script src="/utils.js" defer {};
        script src="/authorsied_page_header.js" defer {};
        (header())
        #"spending-header" ."bg-1" {
            h1 {"Total Spent: " h1 #total {(format!("£{total:0.2}"))}} br;
            h1 {"Total In: " h1 #incomming {(format!("£{total_in:0.2}"))}} br;
            h1 {"Total Out: " h1 #outgoing {(format!("£{:0.2}", total_out))}} br;
        }
        (navigation_bar())
        (content(&transaction_list, &query_params))
    };
    page(wrapped)
}

pub fn page(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        (page_css())
        body {(content)}
    }
}

fn page_css() -> Css {
    Css::from((
        r#" 
        body {
            background-color: #1e1e1e;
            margin: 0;
            font-family: 'Roboto', 'Helvetica', 'Arial', 'sans-serif';
        }
        .bg-1 {
            background-color: {bg-1-colour};
            color: {white};
            margin: 2rem;
            border-radius: 12px;
            font-size: 1rem;
            box-shadow: rgba(0, 0, 0, 0.2) 0px 2px 4px -1px, rgba(0, 0, 0, 0.14) 0px 4px 5px 0px, rgba(0, 0, 0, 0.12) 0px 1px 10px 0px;
        }
        

        .styled-input {
            align-items: center;
            width: 100%;
            position: relative;
            margin: 0.5rem;
            color: #c2c2c2;
            background-color: #333333; 
            height: 2.5rem;
            border-radius: 4px;    
            border-style: solid;
            border-width: 1px;
            border-color: rgba(255, 255, 255, 0.23);
        }

        .styled-button {
            background-color: rgb(144, 202, 249);
            color: rgba(0, 0, 0, 0.87);
            border: none;
            border-radius: 4px;
            transition: background-color 0.4s ease;
        }

        .styled-button:hover {
            background-color: rgb(66 165 245);
        }

    "#,
        main_theme(),
    ))
}

pub fn main_theme() -> Theme {
    let mut t = Theme::new();
    t.insert(
        "white".to_string(),
        ThemeValue::Colour(Colour::from("#ffffff")),
    );
    t.insert(
        "error-colour".to_string(),
        ThemeValue::Colour(Colour::from("#f4c7c7")),
    );
    t.insert(
        "error-bg-colour".to_string(),
        ThemeValue::Colour(Colour::from("#160b0b")),
    );
    t.insert(
        "bg-1-colour".to_string(),
        ThemeValue::Colour(Colour::from("#272727")),
    );
    t
}

pub fn login_signup_css() -> Css {
    Css::from(
        r#"
#page {
    justify-content: center;
    align-items: center;
    display: flex;
    width: 100vw;
    height: 100vh;
}

#login-signup-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    border-radius: 12px;
    width: 400px;
    height: 450px;
    padding: 2rem;
}

a {
    color: #6200ea;
    text-decoration: none;
                        
}
a:hover { 
    text-decoration: underline;
}
"#,
    )
}

fn error_box_css() -> Css {
    Css::from((
        r#"
    #error {
        background-color: {error-bg-colour};
        color: {error-colour};
        display: none; /* Default to not shown */
        width: 100%;
        height: 2.5rem;
        line-height: 2.5rem;
        text-align: center;
        border-radius: 4px;
        margin: 0.5rem;
    }
"#,
        main_theme(),
    ))
}

fn authorised_page_css() -> Css {
    Css::from((
        r#"
        #spending-header {
            padding: 1rem;
        }
        #incomming {
            color:rgb(0, 112, 0);
        }
        #outgoing {
            color: rgb(112,0,0);
        }
        h1 {
            display: inline;
        }
    "#,
        main_theme(),
    ))
}
