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
        components::{add_transaction, adding_pages_css, dropdown_arrow_svg},
        get_cookie,
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct FilterParams {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
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
    let buyer_vec: Option<Vec<&str>> = query_params
        .buyer
        .as_ref()
        .map(|b_str| b_str.split(",").collect());
    let bf_vec: Option<Vec<&str>> = query_params
        .item_bought_for
        .as_ref()
        .map(|bf_str| bf_str.split(",").collect());

    let transaction_list: Vec<Transaction> = transaction_list
        .iter()
        .filter(|t| {
            let mut within_date_rng = true;
            let mut has_tags = true;
            let mut has_buyer = true;
            let mut has_bought_for = true;

            if let Some(t_time) = t.date {
                if let Some(start_date) = query_params.start {
                    if t_time < start_date {
                        within_date_rng = false;
                    }
                }

                if let Some(end_date) = query_params.end {
                    if t_time > end_date {
                        within_date_rng = false;
                    }
                }
            }
            if let Some(tags) = &tags_vec {
                has_tags = false;
                for tag in tags {
                    if t.tags.contains(&tag.to_string()) {
                        has_tags = true;
                        break;
                    }
                }
            }
            if let Some(buyers) = &buyer_vec {
                has_buyer = buyers.contains(&t.buyer.as_str());
            }
            if let Some(bought_for) = &bf_vec {
                has_bought_for = false;
                for b in bought_for {
                    if t.items.iter().any(|item| item.bought_for == *b) {
                        has_bought_for = true;
                        break;
                    }
                }
            }
            has_buyer && has_tags && has_bought_for && within_date_rng
        })
        .cloned()
        .collect();

    let dates = get_days_data(&transaction_list);
    html! {
        script src="/table.js" defer {}
        (table_css())
        (adding_pages_css())
        (add_transaction())
        div #"adding-popup" {}
        div #"add-container" {
            div #"add-btn"{"+"}
            div #"adding-btns" {
                button #"add-transaction-btn" popovertarget="add-single-transaction"{"1"}
                button {"2"}
                button {"3"}
                button {"4"}
            }
        }
        (filter_section(&query_params))
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

fn filter_section(query_params: &Query<FilterParams>) -> Markup {
    html! {
        div #"filters" ."bg-1" {
            div #"filters-header" {
                h3 {"Filters"}
                div ."expand-icon"{
                    (dropdown_arrow_svg())
                }
            }
            div ."dropdown" {
                input #"start-date" name="start-date" ."styled-input" type="date" placeholder="Start Date";
                input #"end-date" name="end-date" ."styled-input" type="date" placeholder="End Date";
                input #"buyer" name="buyer" ."styled-input" type="text" placeholder="Buyer" value=[&query_params.buyer];
                input #"tags" name="tags" ."styled-input" type="text" placeholder="Tags" value=[&query_params.tags];
                input #"bought-for" name="bought-for" ."styled-input" type="text" placeholder="Bought For" value=[&query_params.item_bought_for];
                button #"filter-button" ."styled-input" ."styled-button" {"Apply"}
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
                    @if !transaction.items.is_empty() {
                        (dropdown_arrow_svg())
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
            let date = match transaction.date {
                Some(d) => d,
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
            z-index: 999;
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
            display: none;
            flex-direction: column-reverse;
            transition: opacity 0.4s ease, display 2s;
            transition-behavior: allow-discrete, ;
            opacity: 0;
            z-index: 999;
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
            max-height: inherit;
        }

        #add-container.open #adding-btns {
            opacity: 100%;
            display: flex;
        }

        .header-row, #filters div {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            margin-left: 1rem;
            margin-right: 1rem;
            align-items: center;
        }

        #filters .dropdown {
            flex-wrap: wrap;
            justify-content: space-between;
        }

        #filters .styled-input {
            width: auto;
            min-width: 250px;
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
            align-items: center;
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
        
        .expand-icon {
            display: inline-block;
            transition: transform 0.4s ease;
            transform: rotate(0deg);
            height: 25px;
        }

        .transaction-container.open .expand-icon, #filters.open .expand-icon {
            transform: rotate(180deg);
        }

    "#,
    )
}
