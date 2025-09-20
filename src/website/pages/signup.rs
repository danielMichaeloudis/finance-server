use maud::{html, Markup};

use crate::website::pages::{error_box_css, login_signup_css};

pub fn signup_page() -> Markup {
    html! {
        (login_signup_css())
        (error_box_css())
        #page {
            #"login-signup-container" ."bg-1" {
                h1 {
                    "Signup"
                }
                #error {}
                input #username ."styled-input" type="text" placeholder="Username *";
                input #email ."styled-input" type="email" placeholder="Email *";
                input #password ."styled-input" type="password" placeholder="Password *";
                input #"conf-password" ."styled-input" type="password" placeholder="Confirm Password *";
                button #"signup-button" ."styled-input" ."styled-button" {"Sign Up"}

                p {
                    "Already have an account? "
                    a href="/login" {"Log in"}
                }
                script src="/signup.js" defer {};

            }
        }
    }
}
