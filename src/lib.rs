use syn::DeriveInput;

mod gen;
mod gen_enum;
mod gen_struct;

#[proc_macro_derive(RandGen, attributes(skip_variant, always_some, custom_rand))]
pub fn rand_gen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let transform = gen::transform(input);

    transform.into()
}
