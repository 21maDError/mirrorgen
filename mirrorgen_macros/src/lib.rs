mod attribute;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Ident};
use attribute::MirrorOptions;

#[proc_macro_derive(Mirror, attributes(mirror))]
pub fn derive_mirror(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let args = match MirrorOptions::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let name = &input.ident;
    let args_name = &args.name;

    if let Some(omit) = args.omit {
        dbg!(omit);
    }

    TokenStream::from(quote! {
        impl Mirror for #name {
            fn hello() {
                println!("Found:/n Struct Name: {}\n Attribute Name: {}", stringify!(#name), stringify!(#args_name));
            }
        }
    })
}
