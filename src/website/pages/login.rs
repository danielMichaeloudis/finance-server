use maud::{html, Markup};

use crate::website::pages::{error_box_css, login_signup_css};

pub fn login_page() -> Markup {
    html! {
        (login_signup_css())
        (error_box_css())
        #page {
            #"login-signup-container"  ."bg-1"{
                h1 {
                    "Login"
                }

                #error {}
                input #username name="username" ."styled-input" type="text" placeholder="Username *";
                input #password name="password" ."styled-input" type="password" placeholder="Password *";
                button #"login-button" ."styled-input" ."styled-button" {"Sign In"}

                p {
                    "Don't have an account? "
                    a href="/signup" {"Sign up"}
                }
                script src="/login.js" defer {};

            }
        }
    }
}
