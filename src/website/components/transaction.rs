use css_helper::Css;
use maud::{html, Markup};

use crate::models::Transaction;

pub fn transaction_popup(
    title: Option<&str>,
    js_with_submit: Option<&str>,
    popover_target: Option<&str>,
    transactions: Option<Vec<Transaction>>,
) -> Markup {
    let title = title.unwrap_or("New Transaction");
    let js_with_submit = match js_with_submit {
        Some(s) => html! {script src=(s) defer{}},
        None => html! {},
    };
    html! {
        (transaction_css())
        script src="/items.js" defer{}
        (js_with_submit)
        div #"transaction" ."bg-1" {
            h1 {(title)}
            button #"close-transaction" ."close-btn" popovertarget=[popover_target] popovertargetaction="hide" {"X"}
            form #"transaction-form" {
                input #"vendor" name="vendor" ."styled-input" type="text" placeholder="Vendor *";
                input #"buyer" name="buyer" ."styled-input" type="text" placeholder="Buyer *";
                input #"cost" name="cost" ."styled-input" type="number" placeholder="Cost *";
                input #"tags" name="tags" ."styled-input" type="text" placeholder="Tags *";
                input #"date" name="date" ."styled-input" type="date" placeholder="Date *";
                h3 {"Items"}
                div #"items" {}
                button #"add-item" ."styled-button" type="button" {"Add Item"}
                button #"submit-add-single" ."styled-input"."styled-button" type="button" {"Submit Transaction"};
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

        #transaction * {
            margin: 0.75rem;
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
        }

        .item-input {
            width: 25%;
            margin: 0.25rem;
        }

    "#,
    )
}
