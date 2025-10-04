use axum::Json;
use css_helper::Css;
use maud::{html, Markup};

use crate::{
    models::{Item, Transaction},
    website::components::close_svg,
};

pub fn transaction_popup(
    title: Option<&str>,
    js_with_submit: Option<&str>,
    transactions: Option<&[Transaction]>,
) -> Markup {
    let title = title.unwrap_or("New Transaction");
    let js_with_submit = match js_with_submit {
        Some(s) => html! {script src=(s) defer{}},
        None => html! {},
    };

    let mut uuid = None;
    let mut vendor = None;
    let mut buyer = None;
    let mut cost = None;
    let mut tags = None;
    let mut date = None;
    let mut items = None;

    if let Some(transactions) = transactions {
        if transactions.len() == 1 {
            uuid = Some(&transactions[0].uuid);
            vendor = Some(&transactions[0].vendor);
            buyer = Some(&transactions[0].buyer);
            cost = Some(&transactions[0].cost);
            tags = Some(&transactions[0].tags);
            date = Some(&transactions[0].date);
            items = Some(&transactions[0].items);
        }
    }

    html! {
        (transaction_css())
        script src="/items.js" defer{}
        (js_with_submit)
        div #"transaction" ."bg-1" {
            h1 {(title)}
            button #"close-transaction" ."close-btn" {(close_svg("40px", "40px"))}
            form #"transaction-form" {
                input #"transaction-uuid" ."uuid" readonly value=[uuid.map(|u| u.expect("Must have Uuid"))];
                input #"transaction-vendor" name="vendor" ."styled-input" type="text" placeholder="Vendor *" value=[vendor];
                input #"transaction-buyer" name="buyer" ."styled-input" type="text" placeholder="Buyer *" value=[buyer];
                input #"transaction-cost" name="cost" ."styled-input" type="number" step="0.01" placeholder="Cost *" value=[cost];
                input #"transaction-tags" name="tags" ."styled-input" type="text" placeholder="Tags *" value=[tags.map(|t| t.join(","))];
                input #"transaction-date" name="date" ."styled-input" type="date" placeholder="Date *" value=[date.map(|d| d.unwrap())];
                h3 {"Items"}
                div #"transaction-items" {
                    @if let Some(items) = items {
                        @for item in items {
                            (item_row(Json::from(Some(item.clone()))))
                        }
                    }
                }
                button #"add-item" ."styled-button" type="button" {"Add Item"}
                button #"submit-transaction" ."styled-input"."styled-button" type="button" {"Submit Transaction"};
                button #"remove-transaction" ."styled-input"."styled-button" type="button" {"Remove Transaction"};
            }
        }
    }
}

pub fn item_row(Json(item): Json<Option<Item>>) -> Markup {
    let (name, price, b_for) = match item {
        Some(i) => (Some(i.name), Some(i.price), Some(i.bought_for)),
        None => (None, None, None),
    };
    html! {
        form ."add-item-row" onsubmit="event.preventDefault();addItem();" {
            input type="submit" style="display:none;";
            input type="text" ."styled-input" ."item-input" ."transaction-item-name" placeholder="Name *" value=[name];
            input type="number" ."styled-input" ."item-input" ."transaction-item-price" placeholder="Price *" step="0.01" onchange="updateCost()" value=[price];
            input type="text" ."styled-input" ."item-input" ."transaction-item-bought-for" placeholder="For *" value=[b_for];
            button ."remove-item-btn" type="button" onclick="document.getElementById('transaction-items').removeChild(event.currentTarget.closest('.add-item-row'));updateCost();" {
                (close_svg("25px", "25px"))
            }
        }
    }
}

fn transaction_css() -> Css {
    Css::from(
        r#"
        #transaction {
            padding: 2rem;
            background-color: #2A2A2A;
            max-height: 75vh;
            width: 400px;
            text-align: center;
            overflow-y: auto;
            z-index: 999;
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
        }

        #add-item {
            background-color: rgba(0 0 0 / 0);
            border-color: rgb(144, 202, 249);
            color: rgb(144, 202, 249);
            border-style: solid;
            border-width: thin;
            padding: 10px;
            transition: background-color 0.4s ease;
        }

        #add-item:hover {
            background-color: rgba(144 202 249 / 0.08);
        }

        .add-item-row {
            display: flex;
            flex-direction: row;
            width: 100%;
            justify-content: space-between;
            align-items: center;
        }

        .item-input {
            width: 25%;
            margin: 0.25rem;
        }

        .remove-item-btn {
            background-color: #00000000;
            border-radius: 50%;
            border-style: none;
            width: 30px;
            height: 30px;
            padding: 0;
        }

        #transaction * {
            margin: 0.75rem;
        }

        .add-item-row * {
            margin: 0.5rem;
        }

        .close-btn {
            cursor: pointer;
            display: flex;
            justify-content: center;
            position: absolute;
            right: 8px;
            top: 8px;
            height: 40px;
            width: 40px;
            background-color: rgba(0 0 0 / 0);
            border-radius: 50%;
            border-style: none;
            line-height: 40px;
            color: #fff;
            transition: background-color 0.4s ease;
        }

        .close-btn:hover {
            background-color: rgba(255 255 255 / 0.08);
        }

        .remove-item-btn {
            margin: 0;
            background-color: #ffffff00;
        }

        .remove-item-btn:hover {
            background-color: #ffffff10
        }
    "#,
    )
}
