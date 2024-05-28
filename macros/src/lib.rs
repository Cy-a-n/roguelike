extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(HasTimer)]
pub fn has_timer_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = input.ident;

    // Ensure the input is a struct with named fields
    let data = match input.data {
        Data::Struct(data) => data,
        _ => {
            return TokenStream::from(quote! {
                compile_error!("HasTimer can only be derived for structs with named fields");
            })
        }
    };

    // Find all fields of type bevy::time::Timer
    let timer_fields: Vec<_> = data
        .fields
        .iter()
        .filter(|field| {
            if let Fields::Named(_) = &data.fields {
                let ty = &field.ty;
                if let syn::Type::Path(type_path) = ty {
                    return type_path
                        .path
                        .segments
                        .iter()
                        .any(|segment| segment.ident == "Timer");
                }
            }
            false
        })
        .collect();

    // Ensure there is exactly one Timer field
    if timer_fields.len() != 1 {
        return TokenStream::from(quote! {
            compile_error!("Struct must have exactly one field of type bevy::time::Timer");
        });
    }

    // Get the identifier of the Timer field
    let timer_field = timer_fields[0].ident.as_ref().unwrap();

    // Generate the implementation of the `HasTimer` trait
    let expanded = quote! {
        impl bevy_utils::timer::HasTimer for #name {
            fn timer(&mut self) -> &mut bevy::time::Timer {
                &mut self.#timer_field
            }
        }
    };

    TokenStream::from(expanded)
}
