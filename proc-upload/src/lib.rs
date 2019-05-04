#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;

mod ls;

#[proc_macro_derive(LS)]
pub fn derive_to_ls(input: TokenStream) -> TokenStream {
    ls::derive_ls(syn::parse_macro_input!(input)).into()
}