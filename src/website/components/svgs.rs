use maud::{html, Markup};

// Colour #848484
// https://www.svgrepo.com/collection/dazzle-line-icons

pub fn settings_svg() -> Markup {
    html! {
        svg width="50px" height="50px" fill="#848484ff" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" {
            g id="SVGRepo_bgCarrier" stroke-width="0";
            g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round";
            g id="SVGRepo_iconCarrier"{
                path d="M20.89,9.78h-.65a1.16,1.16,0,0,1-1-.74V9a1.13,1.13,0,0,1,.22-1.26l.46-.46a1.13,1.13,0,0,0,0-1.58L18.29,4.14a1.13,1.13,0,0,0-1.58,0l-.46.46A1.13,1.13,0,0,1,15,4.82h0a1.16,1.16,0,0,1-.74-1V3.11A1.11,1.11,0,0,0,13.11,2H10.89A1.11,1.11,0,0,0,9.78,3.11v.65a1.16,1.16,0,0,1-.74,1H9A1.13,1.13,0,0,1,7.75,4.6l-.46-.46a1.13,1.13,0,0,0-1.58,0L4.14,5.71a1.13,1.13,0,0,0,0,1.58l.46.46A1.13,1.13,0,0,1,4.82,9V9a1.16,1.16,0,0,1-1,.74H3.11A1.11,1.11,0,0,0,2,10.89v2.22a1.11,1.11,0,0,0,1.11,1.11h.65a1.16,1.16,0,0,1,1,.74v0a1.13,1.13,0,0,1-.22,1.26l-.46.46a1.13,1.13,0,0,0,0,1.58l1.57,1.57a1.13,1.13,0,0,0,1.58,0l.46-.46A1.13,1.13,0,0,1,9,19.18H9a1.16,1.16,0,0,1,.74,1v.65A1.11,1.11,0,0,0,10.89,22h2.22a1.11,1.11,0,0,0,1.11-1.11v-.65a1.16,1.16,0,0,1,.74-1h0a1.13,1.13,0,0,1,1.26.22l.46.46a1.13,1.13,0,0,0,1.58,0l1.57-1.57a1.13,1.13,0,0,0,0-1.58l-.46-.46A1.13,1.13,0,0,1,19.18,15v0a1.16,1.16,0,0,1,1-.74h.65A1.11,1.11,0,0,0,22,13.11V10.89A1.11,1.11,0,0,0,20.89,9.78ZM12,16a4,4,0,1,1,4-4A4,4,0,0,1,12,16Z";
            }
        }
    }
}

pub fn dropdown_arrow_svg() -> Markup {
    html! {
        svg width="25px" height="25px" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="#848484" transform="rotate(0)"{
            g id="SVGRepo_bgCarrier" stroke-width="0";
            g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round" stroke="#CCCCCC" stroke-width="0.192";
            g id="SVGRepo_iconCarrier"{
                path d="M6 9L12 15L18 9" stroke="#848484" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round";
            }
        }
    }
}

pub fn close_svg(width: &str, height: &str) -> Markup {
    html! {
        svg width=(width) height=(height) viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="#848484" style="margin:0;"{
            g id="SVGRepo_bgCarrier" stroke-width="0";
            g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round";
            g id="SVGRepo_iconCarrier" {
                path d="M6 6L18 18M18 6L6 18" stroke="#848484" stroke-width="2" stroke-linecap="round" stroke-linejoin="round";
            }
        }
    }
}

pub fn add_transaction_svg() -> Markup {
    html! {
        svg height="24px" width="24px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="#848484"{
            g id="SVGRepo_bgCarrier" stroke-width="0";
            g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round";
            g id="SVGRepo_iconCarrier"{
                path d="M21 5L19 12H7.37671M20 16H8L6 3H3M16 5.5H13.5M13.5 5.5H11M13.5 5.5V8M13.5 5.5V3M9 20C9 20.5523 8.55228 21 8 21C7.44772 21 7 20.5523 7 20C7 19.4477 7.44772 19 8 19C8.55228 19 9 19.4477 9 20ZM20 20C20 20.5523 19.5523 21 19 21C18.4477 21 18 20.5523 18 20C18 19.4477 18.4477 19 19 19C19.5523 19 20 19.4477 20 20Z" stroke="#848484" stroke-width="2" stroke-linecap="round" stroke-linejoin="round";
            }
        }
    }
}

pub fn edit_svg() -> Markup {
    html! {
        svg height="24px" width="24px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="#848484"{
            g id="SVGRepo_bgCarrier" stroke-width="0";
            g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round";
            g id="SVGRepo_iconCarrier"{
                path d="M11 4H7.2C6.0799 4 5.51984 4 5.09202 4.21799C4.71569 4.40974 4.40973 4.7157 4.21799 5.09202C4 5.51985 4 6.0799 4 7.2V16.8C4 17.9201 4 18.4802 4.21799 18.908C4.40973 19.2843 4.71569 19.5903 5.09202 19.782C5.51984 20 6.0799 20 7.2 20H16.8C17.9201 20 18.4802 20 18.908 19.782C19.2843 19.5903 19.5903 19.2843 19.782 18.908C20 18.4802 20 17.9201 20 16.8V12.5M15.5 5.5L18.3284 8.32843M10.7627 10.2373L17.411 3.58902C18.192 2.80797 19.4584 2.80797 20.2394 3.58902C21.0205 4.37007 21.0205 5.6364 20.2394 6.41745L13.3774 13.2794C12.6158 14.0411 12.235 14.4219 11.8012 14.7247C11.4162 14.9936 11.0009 15.2162 10.564 15.3882C10.0717 15.582 9.54378 15.6885 8.48793 15.9016L8 16L8.04745 15.6678C8.21536 14.4925 8.29932 13.9048 8.49029 13.3561C8.65975 12.8692 8.89125 12.4063 9.17906 11.9786C9.50341 11.4966 9.92319 11.0768 10.7627 10.2373Z" stroke="#848484" stroke-width="2" stroke-linecap="round" stroke-linejoin="round";
            }
        }
    }
}
