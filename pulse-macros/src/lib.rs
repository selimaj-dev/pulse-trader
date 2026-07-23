use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemEnum, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn pwp(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::Item);

    match input {
        syn::Item::Struct(s) => expand_struct(s),
        syn::Item::Enum(e) => expand_enum(e),
        _ => {
            return syn::Error::new_spanned(input, "p_com only supports structs and enums")
                .to_compile_error()
                .into();
        }
    }
}

fn expand_struct(mut input: ItemStruct) -> TokenStream {
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

    TokenStream::from(quote! {
        #[derive(Debug, Clone)]
        #input

        impl PulseWire for #name {
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
                        #field_names2: PulseWire::from_com(com),
                    )*
                }
            }
        }
    })
}

fn expand_enum(input: ItemEnum) -> TokenStream {
    let name = &input.ident;

    let to_com = input.variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        let tag = i as u8;

        match &variant.fields {
            Fields::Unit => quote! {
                Self::#ident => {
                    vec.push(#tag);
                }
            },

            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => quote! {
                Self::#ident(v) => {
                    vec.push(#tag);
                    vec.extend(v.to_com());
                }
            },

            Fields::Named(fields) => {
                let names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

                let names2 = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

                quote! {
                    Self::#ident { #( #names ),* } => {
                        vec.push(#tag);
                        #( vec.extend(#names2.to_com()); )*
                    }
                }
            }

            _ => {
                panic!("tuple variants with >1 field are not supported");
            }
        }
    });

    let from_com = input.variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        let tag = i as u8;

        match &variant.fields {
            Fields::Unit => quote! {
                #tag => Self::#ident,
            },

            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                quote! {
                    #tag => Self::#ident(PulseWire::from_com(com)),
                }
            }

            Fields::Named(fields) => {
                let names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

                quote! {
                    #tag => Self::#ident {
                        #(
                            #names: PulseWire::from_com(com),
                        )*
                    },
                }
            }

            _ => panic!("tuple variants with >1 field are not supported"),
        }
    });

    TokenStream::from(quote! {
        #[derive(Debug, Clone)]
        #input

        impl PulseWire for #name {
            fn to_com(&self) -> Vec<u8> {
                let mut vec = Vec::new();

                match self {
                    #( #to_com )*
                }

                vec
            }

            fn from_com(com: &mut Vec<u8>) -> Self {
                let kind = com.remove(0);

                match kind {
                    #( #from_com )*
                    _ => panic!("invalid {} discriminant {}", stringify!(#name), kind),
                }
            }
        }
    })
}
