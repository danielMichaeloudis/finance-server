use css_helper::Css;
use maud::{html, Markup};

use crate::website::components::settings_svg;

pub fn header() -> Markup {
    html! {
        (header_css())
        #header ."bg-1" {
            h1 #title{
                "Ledgerly"
            }
            div #"menu-container"{
                div #"menu-btn" {(settings_svg())}
                div #"menu-dropdown" ."bg-1"  {
                    a #export{"Export Data"}
                    a #logout {"Log Out"}
                }
            }
        }
    }
}

fn header_css() -> Css {
    Css::from(
        r#"
    
    #header {
        display: flex;
        flex-direction: row;
        align-items: center;
        border-radius: 0;
        position: sticky;
        top: 0;
        right: 0;
        margin: 0;
        padding-left: 4rem;
        padding-right: 4rem;
        z-index: 999;
    }

    #header #title {
        flex-grow: 1;
    }

    #menu-container {
        position: relative;
        display: inline-block;
    }

    #menu-btn {
        border-radius: 6px;
        padding: 0 1rem;
        display: inline-block;
        transition: background-color 0.4s ease;
    }

    #menu-container:hover #menu-btn {
        background-color: #ffffff10
    }

    #menu-dropdown {
        display: none; 
        position: absolute;
        right: 0;
        top: 100%; 
        min-width: 160px;
        border-radius: 6px;
        z-index: 1000;
        transition: opacity 0.4s ease, display 2s;
        transition-behavior: allow-discrete, ;
        opacity: 0;
        margin: 0;
    }

    #menu-dropdown a {
        display: block;
        padding: 10px 14px;
        transition: background-color 0.4s ease;
    }

    #menu-dropdown.open{
        display: block;
        opacity: 100%;
    }

    #menu-dropdown a:hover {
        background-color: #ffffff10
    }

    "#,
    )
}
