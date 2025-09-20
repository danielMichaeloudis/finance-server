use maud::{html, Markup};

use crate::website::Css;

pub fn navigation_bar() -> Markup {
    html! {
        (nav_css())
        script src="/nav.js" defer{}
        #"nav-ext"{
            #"nav-container" {
                a #"home-btn" ."nav-button" href="/" {"Analytics"}
                a #"table-btn" ."nav-button" href="/table" {"Table"}
            }
        }
    }
}

fn nav_css() -> Css {
    Css::from(
        r#"
            #nav-ext {
                width: 100%;
            }

            #nav-container {
                display: flex;
                justify-content: space-evenly;
                height: 75px;
                width: 50%;
                margin: auto;
            }

            .nav-button {
                border-bottom-width: 2px;
                border-bottom-color: #141414;
                padding: 0 2rem;
                line-height: 75px;
                color: #ffffff;
                text-decoration-line: none;
            }

        "#,
    )
}
