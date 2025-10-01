use css_helper::Css;

mod buyers_card;
pub use buyers_card::buyers_card;

pub fn card_css() -> Css {
    Css::from(
        r#"
    .card {
        max-width: 225px;
        padding: 0.5rem;
        margin: 1rem;
    }

    .card-title {
        font-size: 18px;
        padding: 0.5rem;
        margin-top: 0;
        margin-bottom: 0.5rem;
        border-bottom-style: solid;
        border-bottom-color: #343434;
    }
    "#,
    )
}
