mod attribute;
mod error;

use attribute::MirrorArgs;
use error::CollectError;
use proc_macro::{Ident, TokenStream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{Error, Fields, ItemStruct, parse_macro_input};

#[proc_macro_derive(Mirror, attributes(mirror))]
pub fn derive_mirror(input: TokenStream) -> TokenStream {
    let mut errors: Option<Error> = None;

    let input = parse_macro_input!(input as ItemStruct);

    if !matches!(input.fields, Fields::Named(_)) {
        errors.push_error(Error::new_spanned(
            &input,
            "#[derive(Mirror)] only works on structs with named fields",
        ));
    }

    let fields: Vec<_> = if let Fields::Named(ref fields) = input.fields {
        fields.named.iter().map(|f| &f.ident).collect()
    } else {
        Vec::new()
    };

    let args: MirrorArgs = match input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("mirror"))
    {
        Some(v) => match v.parse_args() {
            Ok(v) => v,
            Err(e) => {
                errors.push_error(e);
                return TokenStream::new();
            }
        },
        None => {
            errors.push_error(Error::new_spanned(
                &input,
                "`#[mirror]` attribute is required",
            ));
            return TokenStream::new();
        }
    };

    let mut omits = HashSet::new();

    if let Some(omit) = &args.omit {
        for item in omit {
            if !fields.iter().any(|f| f.as_ref() == Some(item)) {
                errors.push_error(Error::new_spanned(item, "field not found"))
            } else if omits.contains(item) {
                errors.push_error(Error::new_spanned(item, "field already omitted"))
            } else {
                omits.insert(item);
            }
        }
    }

    let mut renames = HashMap::new();
    let mut from_s = HashSet::new();
    let mut to_s = HashSet::new();

    if let Some(rename) = &args.rename {
        for (from, to) in rename {
            if !fields.iter().any(|f| f.as_ref() == Some(from)) {
                errors.push_error(Error::new_spanned(from, "field not found"))
            } else if fields.iter().any(|f| f.as_ref() == Some(to)) {
                errors.push_error(Error::new_spanned(to, "field already exists"))
            } else if from_s.contains(from) {
                errors.push_error(Error::new_spanned(from, "field already renamed"));
            } else if to_s.contains(to) {
                errors.push_error(Error::new_spanned(to, "field already used to rename"));
            } else {
                renames.insert(from, to);
                from_s.insert(from);
                to_s.insert(to);
            }
        }
    }

    if let Some(error) = errors {
        return error.to_compile_error().into();
    }

    dbg!(omits);
    dbg!(renames);

    TokenStream::new()
}
