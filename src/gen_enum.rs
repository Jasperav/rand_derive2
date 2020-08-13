use crate::gen::{
    generated_values_for_named_fields, generated_values_for_unnamed_fields, TraitMethods,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields};

pub fn generate(name: &Ident, trait_methods: &mut TraitMethods, de: DataEnum) -> TokenStream {
    let variants = de
        .variants
        .into_iter()
        // Filter out variants annotated with skip_variant
        .filter(|v| !proc_macro2_helper::attributes_contains(&v.attrs, "skip_variant"))
        .collect::<Vec<_>>();
    let variants_len = variants.len();
    let range: Vec<_> = (0..variants_len).collect();

    let ts = variants
        .into_iter()
        .map(|v| {
            let fields = v.fields;
            let ident = v.ident;
            let prefix = quote! {
                #name::#ident
            };

            if fields.is_empty() {
                prefix
            } else {
                match fields {
                    Fields::Named(n) => {
                        let ts = generated_values_for_named_fields(name, n, trait_methods);

                        quote! {
                            #prefix { #(#ts),* }
                        }
                    }
                    Fields::Unnamed(u) => {
                        let ts = generated_values_for_unnamed_fields(name, u, trait_methods);

                        quote! {
                            #prefix (#(#ts),* )
                        }
                    }
                    Fields::Unit => panic!(),
                }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        let random_val = rand::thread_rng().gen_range(0, #variants_len);

        match random_val {
            #(#range => #ts,)*
            _ => panic!()
        }
    }
}
