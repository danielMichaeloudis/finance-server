use css_helper::Css;
use maud::{html, Markup};

pub fn add_transaction() -> Markup {
    html! {
        (add_single_transaction_css())
        script src="/items.js" defer{}
        script src="/submit_transaction.js" defer{}
        div #"add-single-transaction" ."bg-1" popover {
            h1 {"New Transaction"}
            button #"close-add-single-transaction" ."close-btn" popovertarget="add-single-transaction" popovertargetaction="hide" {"X"}
            form {
                input #"add-single-vendor" name="vendor" ."styled-input" type="text" placeholder="Vendor *";
                input #"add-single-buyer" name="buyer" ."styled-input" type="text" placeholder="Buyer *";
                input #"add-single-cost" name="cost" ."styled-input" type="number" placeholder="Cost *";
                input #"add-single-tags" name="tags" ."styled-input" type="text" placeholder="Tags *";
                h3 {"Items"}
                div #"items" {}
                button #"add-item" ."styled-button" type="button" {"Add Item"}
                button #"submit-add-single" ."styled-input"."styled-button" type="button" {"Submit Transaction"};
            }
        }
    }
}

fn add_single_transaction_css() -> Css {
    Css::from(
        r#"
        #add-single-transaction {
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

        #add-single-transaction * {
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

        .item-row {
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
