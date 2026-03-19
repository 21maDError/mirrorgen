use proc_macro2::Span;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

pub struct MirrorArgs {
    pub name: Ident,
    pub omit: Option<Vec<Ident>>,
    pub rename: Option<Vec<(Ident, Ident)>>,
}

impl Parse for MirrorArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name: Option<Ident> = None;
        let mut omit: Vec<Ident> = vec![];
        let mut rename: Vec<(Ident, Ident)> = vec![];

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "name" => {
                    let lit: LitStr = input.parse()?;
                    name = Some(Ident::new(&lit.value(), Span::call_site()));
                }
                "omit" => {
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        omit.push(content.parse()?);
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }
                "rename" => {
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        let from: Ident = content.parse()?;
                        content.parse::<Token![=>]>()?;
                        let to: Ident = content.parse()?;
                        rename.push((from, to));
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        &key,
                        format!("unknown mirror argument `{other}`"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MirrorArgs {
            name: name.ok_or_else(|| input.error("`name` is required"))?,
            omit: if omit.is_empty() { None } else { Some(omit) },
            rename: if rename.is_empty() {
                None
            } else {
                Some(rename)
            },
        })
    }
}
