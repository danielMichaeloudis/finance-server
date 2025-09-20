use css_macro::css;
use maud::{html, Markup};

pub fn error_msg() -> Markup {
    html! {#error {}}
}

const ERROR_BOX_CSS: &str = css!(
    r#"
    #error {
        background-color: {error-bg-colour};
        color: {error-colour};
        display: none; /* Default to not shown */
        width: 100%;
        height: 2.5rem;
        line-height: 2.5rem;
        text-align: center;
        border-radius: 4px;
        margin: 0.5rem;
    }
"#
);
