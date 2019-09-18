extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(TurtleContainer, attributes(turtle))]
pub fn derive_turtle_container(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let mut gen = None;

    if let Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            for field in fields.named.iter() {
                for attr in field.attrs.iter() {
                    if attr.path.is_ident("turtle") {
                        let field_type = &field.ty;
                        let field_ident = &field.ident;
                        let struct_ident = &input.ident;

                        gen = Some(quote! {
                            impl dcc_lsystem::turtle::TurtleContainer for #struct_ident {
                                type Item = <#field_type as dcc_lsystem::turtle::MovingTurtle>::Item;

                                fn inner(&self) -> &dyn dcc_lsystem::turtle::MovingTurtle<Item = Self::Item> {
                                    &self.#field_ident
                                }
                            }
                        });
                    }
                }
            }
        }
    }

    if let Some(gen) = gen {
        gen.into()
    } else {
        TokenStream::new()
    }
}
