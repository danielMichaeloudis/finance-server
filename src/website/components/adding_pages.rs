use css_helper::Css;
use maud::{html, Markup};

use crate::website::components::transaction::transaction_popup;

pub fn add_transaction() -> Markup {
    html! {
        div #"add-single-transaction" popover {
            (
                transaction_popup(
                    Some("New Transaction"),
                    Some("/add_single_transaction.js"),
                    Some("add-single-transaction"),
                    None,
                )
            )
        }
    }
}

pub fn adding_pages_css() -> Css {
    Css::from(
        r#"
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
    "#,
    )
}
