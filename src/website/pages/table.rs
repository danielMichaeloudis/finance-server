use std::collections::VecDeque;

use axum::extract::{Query, Request};
use chrono::{Datelike, Month, NaiveDate};
use css_helper::Css;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};

use crate::{
    api_bridge::ApiBridge,
    models::Transaction,
    website::{
        components::{add_transaction, adding_pages_css},
        get_cookie,
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct FilterParams {
    tags: Option<String>,
    buyer: Option<String>,
    item_bought_for: Option<String>,
}

pub async fn table_page(req: Request) -> Markup {
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

    let tags_vec: Option<Vec<&str>> = query_params
        .tags
        .as_ref()
        .map(|tag_str| tag_str.split(",").collect());

    let transaction_list: Vec<Transaction> = transaction_list
        .iter()
        .filter(|t| {
            let mut has_tags = true;
            if let Some(tags) = &tags_vec {
                has_tags = false;
                for tag in tags {
                    if t.tags.contains(&tag.to_string()) {
                        has_tags = true;
                    }
                }
            }
            let has_buyer = match &query_params.buyer {
                Some(buyer) => buyer == &t.buyer,
                None => true,
            };
            let has_bought_for = match &query_params.item_bought_for {
                Some(bought_for) => t.items.iter().any(|item| &item.bought_for == bought_for),
                None => true,
            };
            has_buyer && has_tags && has_bought_for
        })
        .cloned()
        .collect();

    let dates = get_days_data(&transaction_list);
    html! {
        script src="/table.js" defer {}
        (table_css())
        (adding_pages_css())
        div #"adding-popup" {}
        div #"add-container" {
            div #"add-btn"{"+"}
            div #"adding-btns" {
                button #"add-transaction-btn" {"1"}
                button {"2"}
                button {"3"}
                button {"4"}
            }
        }
        div ."bg-1"{
            @for (date, mut transactions) in dates {
                ."header-row" {
                    h5 {(format_date(&date))}
                    h5 {(format!("£{:.2}",transactions.iter().fold(0.0, |acc, t| acc + t.cost)))}
                }
                hr;
                ul {
                    @if let Some(transaction) = transactions.pop_front() {
                        (transaction_row(&transaction))
                    }
                    @for transaction in transactions {
                        li ."transaction-divider" {}
                        (transaction_row(&transaction))
                    }
                }
                hr;
            }
        }
    }
}

fn transaction_row(transaction: &Transaction) -> Markup {
    html! {
        div ."transaction-container"{
            div ."transaction-row"{
                div .icon{
                    span {}
                }
                div .vendor{
                    p {(transaction.vendor)}
                }
                div ."tags" {
                    div {
                        @for tag in &transaction.tags {
                            button ."pill" onclick=(format!("location.assign('/table?tags={}')", tag)) {"#"(tag)}
                        }
                    }
                }
                div ."buyer"{
                    div {
                        button ."pill" onclick=(format!("location.assign('/table?buyer={}')", transaction.buyer)) {(transaction.buyer)}
                    }
                }
                div ."transaction-cost"{
                    p {(format!("£{:.2}",transaction.cost))}
                }
                div ."expand-icon"{
                    span ."expand-marker"{
                        @if !transaction.items.is_empty() {
                            "V"
                        }
                        //svg{}
                    }
                }
            }
            div ."dropdown"{
                @for item in &transaction.items {
                    div ."item-row" {
                        div ."item-name" {(item.name)}
                        div {button ."item_bought_for" ."pill" {(item.bought_for)}}
                        div ."item-price" {(format!("£{:.2}", item.price))}
                    }
                }
            }
        }
    }
}

fn format_date(date: &NaiveDate) -> String {
    let current_year = chrono::Utc::now().year();
    format!(
        "{} {} {} {}",
        date.weekday(),
        date.day(),
        Month::try_from(date.month() as u8).unwrap().name(),
        if date.year() != current_year {
            date.year().to_string()
        } else {
            "".to_string()
        }
    )
}

fn get_days_data(transactions: &[Transaction]) -> Vec<(NaiveDate, VecDeque<Transaction>)> {
    if transactions.is_empty() {
        return vec![];
    }
    let mut t = transactions.iter().fold(
        vec![],
        |mut acc: Vec<(NaiveDate, VecDeque<Transaction>)>, transaction| {
            let date = match transaction.transaction_timestamp {
                Some(d) => d.date(),
                None => return acc,
            };
            let date_entry = acc.iter_mut().find(|e| e.0 == date);
            let date_entry = match date_entry {
                Some(d) => d,
                None => {
                    acc.push((date, VecDeque::from(vec![])));
                    acc.last_mut().unwrap()
                }
            };
            date_entry.1.push_back(transaction.clone());
            acc
        },
    );
    t.sort_by(|a, b| b.0.cmp(&a.0));
    t
}

fn table_css() -> Css {
    Css::from(
        r#"

        #add-container {
            padding: 1rem;
            position: fixed;
            bottom: 64px;
            right: 16px;
            display: flex;
            flex-direction: column-reverse;
        }

        #add-btn {
            border-radius: 50%;
            background-color: rgb(144, 202, 249);
            border-color: rgba(0, 0, 0, 0.87);
            height: 56px;
            width: 56px;
            text-align: center;
            line-height: 56px;
            font-size: 24px;
            transition: transform 0.2s ease;
            transform: rotate(0deg);
        }

        #add-container.open #add-btn {
            transform: rotate(45deg);
            background-color: rgb(66, 165, 245)
        }

        #adding-btns {
            margin: 0.5rem;
            display: flex;
            flex-direction: column-reverse;
            transition: opacity 0.4s ease;
            opacity: 0
        }

        #adding-btns button {
            margin-bottom: 0.5rem;
            width: 40px;
            height: 40px;
            background-color: rgb(18, 18, 18);
            border-radius: 50%;
            border-color: rgba(0, 0, 0, 0.87);
            border-style: solid;
            color: #fff;
        }

        #add-container.open #adding-btns {
            opacity: 100%
        }

        .header-row {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            margin-left: 1rem;
            margin-right: 1rem;
        }

        ul {
            margin: 0;
        }

        hr {
            border-color: rgba(255, 255, 255, 0.12);
            margin: 0;
        }

        .transaction-divider {
            border-color: rgba(255, 255, 255, 0.12);
            border-bottom-style: solid;
            border-bottom-width: thin;
            list-style: none;
            width: calc(100% * 14/15);
            margin-left: calc(100% * 1/15);
        }

        .transaction-row {
            display: flex;
            flex-direction: row;
            width: 100%;
        }

        .transaction-row div, .item-row div {
            margin: 1rem;
        }

        .icon {
            width: calc(100% * 1/15);
        }

        .vendor {
            flex-grow: 1;
        }

        .tags {
            width: auto;
            height: 100%;
        }

        .buyer {
            width: auto;
        }

        .transaction-cost {
            width: calc(100% * 2/15);
        }

        .dropdown {
            overflow: hidden;
            max-height: 0;
            opacity: 0;
            transition: max-height 0.5s ease, opacity 0.5s ease;
        }

        .item-row {
            display: flex;
            flex-direction: row;
            width: calc(100% * 12/15);
            margin-left: calc(100% * 2/15);
        }

        .item-name {
            flex-grow: 1;
        }

        .item-bought-for {
            width: auto;
        }

        .item-price {
            width: calc(100% * 2/15);
        }

        .expand-icon {
            padding: 1rem;
        }

        .pill {
            padding: 6px 14px;
            border-radius: 999px;
            background-color: rgba(255, 255, 255, 0.16);
            cursor: pointer;
            border-style: none;
            color: #fff;
        }

        .pill:hover {
            background-color: rgba(255, 255, 255, 0.24);
        }
        
        .expand-marker {
            display: inline-block;
            transition: transform 0.4s ease;
            transform: rotate(0deg);
        }

        .transaction-container.open .expand-marker {
            transform: rotate(180deg);
        }

    "#,
    )
}
