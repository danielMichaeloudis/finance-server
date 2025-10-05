use std::collections::HashMap;

use axum::extract::Query;
use chrono::{Datelike, Month, NaiveDate};
use css_helper::Css;
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    models::Transaction,
    website::{
        components::{add_transaction_svg, dropdown_arrow_svg, edit_svg, filter_section},
        pages::FilterParams,
    },
};

pub fn table_page(
    transaction_map: &HashMap<Uuid, Transaction>,
    query_params: &Query<FilterParams>,
) -> Markup {
    let dates = get_days_data(transaction_map);
    html! {
        script src="/table.js" defer {}
        (table_css())
        div #"adding-popup" {}
        div #"add-container" {
            div #"add-btn"{"+"}
            div #"adding-btns" {
                button #"add-transaction-btn" ."bg-1"{
                    (add_transaction_svg())
                }
            }
        }
        (filter_section(query_params))
        div ."bg-1"{
            @for (date, transactions) in dates {
                ."header-row" {
                    h5 {(format_date(&date))}
                    h5 {(format!("£{:.2}",transactions.iter().fold(0.0, |acc, (_, t)| acc + t.cost)))}
                }
                hr;
                ul {
                    @for (i, (uuid, transaction)) in transactions.iter().enumerate() {
                        @if i != 0 {
                        li ."transaction-divider" {}
                        }
                        (transaction_row(&uuid, &transaction))
                    }
                }
                hr;
            }
        }
    }
}

fn transaction_row(uuid: &Uuid, transaction: &Transaction) -> Markup {
    html! {
        div ."transaction-container"{
            div ."transaction-row"{
                div ."uuid" {
                    (uuid)
                }
                div ."icon"{
                    span {}
                }
                div ."vendor"{
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
                button ."edit-btn" onclick="editTransaction(event)" {
                    (edit_svg())
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

fn get_days_data(
    transactions: &HashMap<Uuid, Transaction>,
) -> Vec<(NaiveDate, HashMap<Uuid, Transaction>)> {
    if transactions.is_empty() {
        return vec![];
    }
    let mut t = transactions.iter().fold(
        vec![],
        |mut acc: Vec<(NaiveDate, HashMap<Uuid, Transaction>)>, (uuid, transaction)| {
            let date = match transaction.date {
                Some(d) => d,
                None => return acc,
            };
            let date_entry = acc.iter_mut().find(|e| e.0 == date);
            let date_entry = match date_entry {
                Some(d) => d,
                None => {
                    acc.push((date, HashMap::new()));
                    acc.last_mut().unwrap()
                }
            };
            date_entry.1.insert(*uuid, transaction.clone());
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
            transition-behavior: allow-discrete;
            opacity: 0;
            z-index: 999;
        }

        #adding-btns button {
            width: 40px;
            height: 40px;
            transition: background-color 0.4s ease;
            border-radius: 50%;
            border-style: none;
            color: #fff;
            max-height: inherit;
            margin: 0 0 0.5rem 0;
            padding: 0;
            display: flex;   
            justify-content: center; 
            align-items: center;
        }

        #adding-btns button:hover {
            background-color: #353535;
        }

        #add-container.open #adding-btns {
            opacity: 100%;
            display: flex;
        }

        .header-row {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            margin-left: 1rem;
            margin-right: 1rem;
            align-items: center;
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

        .edit-btn {
            height: 35px;
            width: 35px;
            border-style: none;
            border-radius: 50%;
            background-color: #00000000;
            transition: background-color 0.4s ease;
            display: flex;   
            justify-content: center;
            align-items: center; 
            margin: 1rem;
            opacity: 0;
        }

        .transaction-container:hover * .edit-btn {
            opacity: 100%;
        }

        .edit-btn:hover {
            background-color: rgba(255, 255, 255, 0.24);
        }
        
        .expand-icon {
            display: inline-block;
            transition: transform 0.4s ease;
            transform: rotate(0deg);
            height: 25px;
        }

        .transaction-container.open .expand-icon{
            transform: rotate(180deg);
        }

        .uuid {
            display: none;
        }

    "#,
    )
}
