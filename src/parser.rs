use crate::parser::Customize::{AlwaysNone, AlwaysSome, Custom, Empty, Panic, Skip};
use proc_macro2::TokenStream;
use quote::ToTokens;
#[allow(unused_imports)]
use std::str::FromStr;
use syn::parse::{Parse, ParseBuffer};
use syn::Attribute;

#[allow(clippy::ptr_arg)]
pub(crate) fn attrs_to_customizes(attrs: &Vec<Attribute>) -> Vec<Customize> {
    attrs
        .clone()
        .into_iter()
        .filter_map(|a| {
            if !proc_macro2_helper::attribute_contains(&a, "rand_derive") {
                return None;
            }

            let tokens = &a.meta.require_list().unwrap().tokens;

            Some(syn::parse2::<Customize>(tokens.to_token_stream()).unwrap())
        })
        .collect()
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Customize {
    Skip,
    AlwaysSome,
    AlwaysNone,
    Custom,
    Panic,
    Default,
    Empty,
    Fixed(String),
}

impl Parse for Customize {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let ident: proc_macro2::Ident = syn::parse::Parse::parse(input)?;
        let ident = ident.to_string();
        let result = if ident == "fixed" {
            // This is the equal sign
            let _: proc_macro2::Punct = syn::parse::Parse::parse(input)?;

            // This is the actual value
            let lit: proc_macro2::Literal = syn::parse::Parse::parse(input)?;

            // There are still leading and trailing quotes, this needs to be removed
            let replaced_value = lit.to_string().replace('\"', "");

            Customize::Fixed(replaced_value)
        } else {
            match ident.as_str() {
                "skip" => Skip,
                "some" => AlwaysSome,
                "none" => AlwaysNone,
                "custom" => Custom,
                "panic" => Panic,
                "default" => Customize::Default,
                "empty" => Empty,
                _ => panic!("Unknown customization: {}", ident),
            }
        };

        Ok(result)
    }
}

pub(crate) fn has_customize(customizes: &[Customize], customize: Customize) -> bool {
    customizes.iter().any(|c| c == &customize)
}

pub(crate) fn fixed_value(customizes: &[Customize]) -> Option<TokenStream> {
    customizes.iter().find_map(|c| {
        if let Customize::Fixed(fixed) = c {
            Some(TokenStream::from_str(fixed).unwrap())
        } else {
            None
        }
    })
}
