extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use regex_macro::regex;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitStr, Token};

struct RequestInput(syn::Ident, Token!(,), LitStr, syn::Ident);

impl Parse for RequestInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RequestInput(
            input.parse()?,
            input.parse()?,
            input.parse()?,
            input.parse()?,
        ))
    }
}

#[proc_macro]
pub fn use_request(tokens: TokenStream) -> TokenStream {
    let RequestInput(cx, _, str_lit, method) = parse_macro_input!(tokens);

    let deps = extract_deps(str_lit.value().as_str());
    quote!(dioxus_use_request::use_request(&#cx, (#(#deps),*), format!(#str_lit), dioxus_use_request::RequestMethod::#method)).into()
}

struct GetInput(syn::Ident, Token!(,), LitStr);

impl Parse for GetInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(GetInput(input.parse()?, input.parse()?, input.parse()?))
    }
}

#[proc_macro]
pub fn use_get(tokens: TokenStream) -> TokenStream {
    let GetInput(cx, _, str_lit) = parse_macro_input!(tokens);

    let deps = extract_deps(str_lit.value().as_str());

    quote!(dioxus_use_request::use_get(&#cx, (#(#deps),*), format!(#str_lit))).into()
}

fn extract_deps(url: &str) -> Vec<Ident> {
    let regex = regex!(r"\{.*?\}");
    regex
        .find_iter(&url)
        .map(|m| {
            Ident::new(
                &m.as_str().replace("{", "").replace("}", "")[..],
                Span::call_site(),
            )
        })
        .collect()
}
