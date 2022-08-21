extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use regex_macro::regex;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitStr, Token};

struct MacroInput(syn::Ident, Token!(,), LitStr);

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MacroInput(input.parse()?, input.parse()?, input.parse()?))
    }
}

#[proc_macro]
pub fn use_request(tokens: TokenStream) -> TokenStream {
    let MacroInput(cx, _, str_lit) = parse_macro_input!(tokens);

    let url = str_lit.value();
    let regex = regex!(r"\{.*?\}");
    let deps = regex
        .find_iter(&url)
        .map(|m| {
            Ident::new(
                &m.as_str().replace("{", "").replace("}", "")[..],
                Span::call_site(),
            )
        })
        .collect::<Vec<_>>();

    quote!(dioxus_use_request::use_request(&#cx, (#(#deps),*), format!(#str_lit))).into()
}
