extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn generate_serializable_impl(input: &DeriveInput) -> TokenStream {
    let ident = input.ident.clone();
    let mut read_fields = proc_macro2::TokenStream::new();
    let mut write_fields = proc_macro2::TokenStream::new();
    match &input.data {
        syn::Data::Struct(s) => {
            for f in &s.fields {
                let ident = f.ident.clone().unwrap();

                read_fields.extend(quote! {
                    res.#ident = Serializable::read_from(r)?;
                });
                write_fields.extend(quote! {
                    self.#ident.write_to(w)?;
                });
            }
        }
        _ => panic!("Serializable attr can only be used on structs!"),
    }

    quote! {
        impl Serializable for #ident {
            fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
                let mut res = Self::default();
                #read_fields
                Ok(res)
            }

            fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
                #write_fields

                Ok(())
            }
        }
    }
    .into()
}

#[proc_macro_derive(Serializable)]
pub fn derive_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generated_impl = generate_serializable_impl(&input);

    generated_impl
}
