extern crate proc_macro;

use quote::*;
use syn::parse::*;
use syn::*;
use proc_macro::TokenStream;

struct MacroInput(Ident, LitStr);

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MacroInput(input.parse()?, input.parse()?))
    }
}

#[proc_macro]
pub fn use_request(tokens: TokenStream) -> TokenStream {
    let cx, url = parse_macro_input!(tokens as MacroInput);
    // ...

    quote! {
         use_request(&#cx)
    }
    .into()
}
