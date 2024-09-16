use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Position)]
pub fn derive_position(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = match input.data {
        syn::Data::Struct(s) => {
            let name = input.ident;
            let fields = match s.fields {
                syn::Fields::Named(fields) => fields.named,
                _ => panic!("Position derive only works on structs with named fields"),
            };
            let position_field = fields
                .iter()
                .find(|f| f.ident.as_ref().unwrap() == "position");
            let position_field = match position_field {
                Some(f) => f,
                None => panic!("Position derive only works on structs with a field `position`"),
            };
            let (position_type, inner_type) = match &position_field.ty {
                syn::Type::Path(p) => {
                    let segment = &p.path.segments[0];
                    let position_type = &segment.ident;
                    let inner_type = match &segment.arguments {
                        syn::PathArguments::AngleBracketed(a) => {
                            let inner_type = &a.args[0];
                            let inner_type = match inner_type {
                                syn::GenericArgument::Type(t) => t,
                            _ => panic!("Position derive only works on structs with a field `position` of type with a generic argument and fields `x` and `y`"),
                            };
                            match inner_type {
                                syn::Type::Path(p) => {
                                    let segment = &p.path.segments[0];
                                    &segment.ident
                                }
                                _ => panic!("Position derive only works on structs with a field `position` of type with a generic argument and fields `x` and `y`"),
                            }
                        }
                        _ => panic!("Position derive only works on structs with a field `position` of type with a generic argument and fields `x` and `y`"),
                    };
                    (position_type, inner_type)
                }
                _ => panic!("Position derive only works on structs with a field `position` of type with a generic argument and fields `x` and `y`"),
            };

            // Debugging with cargo expand
            // println!("{:#?}", inner_type);
            quote! {
                impl Position<#inner_type> for #name {
                    fn position(&self) -> #position_type<#inner_type> {
                        self.position
                    }
                    fn set_position(&mut self, position: #position_type<#inner_type>) {
                        self.position.x = position.x;
                        self.position.y = position.y;
                    }
                }
            }
        }
        _ => unimplemented!(),
    };
    proc_macro::TokenStream::from(expanded)
}
