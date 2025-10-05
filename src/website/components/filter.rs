use std::collections::HashMap;

use axum::extract::Query;
use css_helper::Css;
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    models::Transaction,
    website::{components::dropdown_arrow_svg, pages::FilterParams},
};

pub fn filter_section(query_params: &Query<FilterParams>) -> Markup {
    html! {
        (filter_css())
        script src="/filters.js" defer{}
        div #"filters" ."bg-1" {
            div #"filters-header" {
                h3 {"Filters"}
                div ."expand-icon"{
                    (dropdown_arrow_svg())
                }
            }
            form #"filter-form" ."dropdown" {
                input #"filter-start-date" name="start-date" ."styled-input" type="date" placeholder="Start Date";
                input #"filter-end-date" name="end-date" ."styled-input" type="date" placeholder="End Date";
                input #"filter-buyer" name="buyer" ."styled-input" type="text" placeholder="Buyer" value=[&query_params.buyer];
                input #"filter-tags" name="tags" ."styled-input" type="text" placeholder="Tags" value=[&query_params.tags];
                input #"filter-bought-for" name="bought-for" ."styled-input" type="text" placeholder="Bought For" value=[&query_params.item_bought_for];
                button #"filter-button" ."styled-input" ."styled-button" {"Apply"}
            }
        }
    }
}

pub fn filter_transactions(
    transactions: HashMap<Uuid, Transaction>,
    query_params: &Query<FilterParams>,
) -> HashMap<Uuid, Transaction> {
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

    transactions
        .into_iter()
        .filter_map(|(uuid, mut transaction)| {
            let mut within_date_rng = true;
            let mut has_tags = true;
            let mut has_buyer = true;

            if let Some(t_time) = transaction.date {
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
                    if transaction.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)) {
                        has_tags = true;
                        break;
                    }
                }
            }
            if let Some(buyers) = &buyer_vec {
                has_buyer = buyers
                    .iter()
                    .any(|b| b.eq_ignore_ascii_case(&transaction.buyer));
            }

            match &bf_vec {
                Some(bought_for) => {
                    let filtered_items: Vec<crate::models::Item> = transaction
                        .items
                        .into_iter()
                        .filter(|item| {
                            bought_for
                                .iter()
                                .any(|b| b.eq_ignore_ascii_case(&item.bought_for))
                        })
                        .collect();
                    let cost = filtered_items.iter().fold(0.0, |acc, i| acc + i.price);
                    if !filtered_items.is_empty() {
                        transaction.cost = cost;
                        transaction.items = filtered_items;
                        Some((uuid, transaction))
                    } else {
                        None
                    }
                }
                None => match has_buyer && has_tags && within_date_rng {
                    true => Some((uuid, transaction)),
                    false => None,
                },
            }
        })
        .collect()
}

fn filter_css() -> Css {
    Css::from(
        r#"
        #filters div, #filters form {
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

        #filters.open .expand-icon {
            transform: rotate(180deg);
        }

        .dropdown {
            overflow: hidden;
            max-height: 0;
            opacity: 0;
            transition: max-height 0.5s ease, opacity 0.5s ease;
        }
        
        .expand-icon {
            display: inline-block;
            transition: transform 0.4s ease;
            transform: rotate(0deg);
            height: 25px;
        }
    "#,
    )
}
