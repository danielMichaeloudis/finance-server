use std::{collections::HashMap, fmt::Display};

use maud::{Render, html};

pub enum ThemeValue {
    String(String),
    Colour(Colour),
}

impl Display for ThemeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ThemeValue::String(val) => val.to_string(),
                ThemeValue::Colour(val) => val.to_string(),
            }
        )
    }
}

pub type Theme = HashMap<String, ThemeValue>;

pub struct Css {
    pub unthemed_css: String,
    pub theme: Option<Theme>,
}

impl Css {
    fn get_themed(&self) -> String {
        let theme = match &self.theme {
            Some(t) => t,
            None => return self.unthemed_css.clone(),
        };
        let mut css = self.unthemed_css.clone();
        for (key, val) in theme.iter() {
            let val: String = val.to_string();
            css = css.replace(&format!("{{{}}}", key).to_string(), &val)
        }
        css
    }
}

impl From<&str> for Css {
    fn from(css: &str) -> Self {
        Self {
            unthemed_css: css.to_owned(),
            theme: None,
        }
    }
}

impl From<(&str, Theme)> for Css {
    fn from((css, theme): (&str, Theme)) -> Self {
        Self {
            unthemed_css: css.to_owned(),
            theme: Some(theme),
        }
    }
}

impl Render for Css {
    fn render(&self) -> maud::Markup {
        html! {
            style {(self.get_themed())}
        }
    }
}

pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f64,
}

impl From<(u8, u8, u8)> for Colour {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self {
            red: rgb.0,
            green: rgb.1,
            blue: rgb.2,
            alpha: 1.0,
        }
    }
}
impl From<(u8, u8, u8, f64)> for Colour {
    fn from(rgba: (u8, u8, u8, f64)) -> Self {
        let alpha = rgba.3;
        let alpha = alpha.clamp(0.0, 1.0);
        Self {
            red: rgba.0,
            green: rgba.1,
            blue: rgba.2,
            alpha,
        }
    }
}
impl From<&str> for Colour {
    fn from(val: &str) -> Self {
        let val = if val.starts_with("#") {
            val.split_at(1).1
        } else {
            val
        };
        let (rg, blue) = val.split_at(4);
        let (red, green) = rg.split_at(2);
        Self {
            red: hex_string_to_int(red),
            green: hex_string_to_int(green),
            blue: hex_string_to_int(blue),
            alpha: 1.0,
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rgb({} {} {} / {:.4})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

fn hex_string_to_int(hex_string: &str) -> u8 {
    u8::from_str_radix(hex_string, 16).unwrap_or(255)
}

#[macro_export]
macro_rules! script {
    ($val:literal) => {
        concat!("<script>", $val, "</script>")
    };
}
