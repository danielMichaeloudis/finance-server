use css_helper::Css;
use maud::html;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Expr, LitStr, Token, parse::Parse, parse_macro_input};

enum CssInput {
    CssOnly(CssOnly),
    CssAndTheme(CssAndTheme),
}

impl Parse for CssInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let css_string = input.parse()?;
        match input.parse::<Token![,]>() {
            Ok(_) => {
                let theme = input.parse()?;
                Ok(Self::CssAndTheme(CssAndTheme { css_string, theme }))
            }
            Err(_) => Ok(Self::CssOnly(CssOnly { css_string })),
        }
    }
}

impl ToTokens for CssInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self {
            CssInput::CssOnly(input) => {
                input.to_tokens(tokens);
            }
            CssInput::CssAndTheme(input) => {
                input.to_tokens(tokens);
            }
        };
    }
}

struct CssOnly {
    css_string: LitStr,
}

impl Parse for CssOnly {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let css_string = input.parse()?;
        Ok(Self { css_string })
    }
}

impl ToTokens for CssOnly {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let css_str = &self.css_string;
        let c = css_helper::Css::from(css_str.value().as_str());
        let ret = html!((c)).0;
        let out = quote! {#ret};
        tokens.extend(out);
    }
}

struct CssAndTheme {
    css_string: LitStr,
    theme: Expr,
}

impl Parse for CssAndTheme {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let css_string = input.parse()?;
        _ = input.parse::<Token![,]>()?;
        let theme = input.parse()?;
        Ok(Self { css_string, theme })
    }
}

impl ToTokens for CssAndTheme {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let css_str = &self.css_string;
        let theme = &self.theme;
        let out = quote! {{
            extern crate css_helper;
            let mut _c = css_helper::Css::from((#css_str, #theme));
            _c
        }};
        tokens.extend(out);
    }
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CssInput);
    quote! {#input}.into()
}
