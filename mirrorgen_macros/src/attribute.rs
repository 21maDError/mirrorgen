use deluxe::ExtractAttributes;
use syn::{Path, Token, parse::{Parse, ParseStream}};

#[derive(Debug)]
pub struct Pair {
    pub from: Path,
    pub to: Path,
}

// Manually implement deluxe::Parse for your helper struct
impl deluxe::Parse for Pair {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let from: Path = input.parse()?;
        let _: Token![=>] = input.parse()?; // Consume the separator
        let to: Path = input.parse()?;
        Ok(Pair { from, to })
    }
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(mirror))]
pub struct MirrorOptions {
    pub name: String,
    pub omit: Option<Vec<Path>>,
    
    // Now this works automatically because Pair implements deluxe::Parse
    pub rename: Option<Vec<Pair>>
}