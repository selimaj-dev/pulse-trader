use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn p_com(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    let name = &input.ident;

    match &mut input.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                field.vis = syn::Visibility::Public(syn::token::Pub::default());
            }
        }
        _ => {
            return syn::Error::new_spanned(input, "p_com only supports structs with named fields")
                .to_compile_error()
                .into();
        }
    }

    let fields = match &input.fields {
        Fields::Named(fields) => &fields.named,
        _ => {
            return syn::Error::new_spanned(input, "p_com only supports structs with named fields")
                .to_compile_error()
                .into();
        }
    };

    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_names2 = fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_types = fields.iter().map(|f| &f.ty);

    TokenStream::from(quote! {
        #[derive(Debug, Clone)]
        #input

        impl PulseCom for #name {
            fn to_com(&self) -> Vec<u8> {
                let mut vec = Vec::new();

                #(
                    vec.extend(self.#field_names.to_com());
                )*

                vec
            }

            fn from_com(com: &mut Vec<u8>) -> Self {
                Self {
                    #(
                        #field_names2: <#field_types>::from_com(com),
                    )*
                }
            }
        }
    })
}
