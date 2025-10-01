use std::collections::HashMap;

use css_helper::Css;
use maud::{html, Markup};

use crate::{models::Transaction, website::components::card_css};

pub fn buyers_card(transactions: &[Transaction]) -> Markup {
    let mut buyers: HashMap<String, f64> = HashMap::new();
    transactions.iter().for_each(|t| {
        if buyers.contains_key(&t.buyer) {
            *buyers.get_mut(&t.buyer).unwrap() += t.cost;
        } else {
            buyers.insert(t.buyer.clone(), t.cost);
        }
    });
    let mut buyers_vec: Vec<_> = buyers.iter().collect();
    buyers_vec.sort_by(|a, b| b.1.total_cmp(a.1));
    html! {
        (buyer_card_css())
        div #"buyers-card" ."bg-1" ."card" {
            p ."card-title" {"Top Buyers"}
            @for (buyer_name, buyer_bought) in buyers_vec {
                div ."card-content-row"{
                    p ."name" {(buyer_name) }
                    p ."cost" {"£" (format!("{buyer_bought:.2}"))}
                }
            }
        }
    }
}

fn buyer_card_css() -> Css {
    Css::from(
        r#"
        #buyers-card .card-content-row {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
        }


    "#,
    )
}
