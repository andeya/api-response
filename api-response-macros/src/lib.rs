use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::Ident;
mod enum_digits;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ErrTypeConstructor)]
pub fn err_type_constructor(_input: TokenStream) -> TokenStream {
    let mut new_type_methods = Vec::new();

    for i in 1000..=4293u16 {
        let flag_value = Literal::u16_unsuffixed(i);
        let new_type_method_ident = Ident::new(&format!("T{i:04}"), Span::call_site());

        new_type_methods.push(quote! {
            #[allow(non_snake_case)]
            #[inline]
            pub const fn #new_type_method_ident(text: &'static str) -> ErrType {
                ErrType { text, flag: #flag_value }
            }
        });
    }

    let expanded = quote! {
        impl ErrType {
            #(#new_type_methods)*
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ErrPathConstructor)]
pub fn err_path_constructor(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let expanded = match ast.ident.to_string().as_str() {
        "ErrPathRoot" => {
            let mut new_root_methods = Vec::new();
            let mut new_parent_methods = Vec::new();

            for i in 0..=99u8 {
                let flag_value = Literal::u8_unsuffixed(i);
                let new_root_method_ident = Ident::new(&format!("X{i:02}"), Span::call_site());
                let new_parent_method_ident = Ident::new(&format!("Y{i:02}"), Span::call_site());

                new_root_methods.push(quote! {
                    #[allow(non_snake_case)]
                    #[inline]
                    pub const fn #new_root_method_ident(name: &'static str) -> ErrPathRoot {
                        ErrPathRoot { name, flag: #flag_value }
                    }
                });
                new_parent_methods.push(quote! {
                    #[allow(non_snake_case)]
                    #[inline]
                    pub const fn #new_parent_method_ident(self, name: &'static str) -> ErrPathParent {
                        ErrPathParent { root: self, name, flag: #flag_value }
                    }
                });
            }

            quote! {
                impl ErrPathRoot {
                    #(#new_root_methods)*

                    #(#new_parent_methods)*
                }
            }
        }

        "ErrPathParent" => {
            let mut new_path_methods = Vec::new();

            for i in 0..=99u8 {
                let flag_value = Literal::u8_unsuffixed(i);
                let new_path_method_ident = Ident::new(&format!("Z{i:02}"), Span::call_site());

                new_path_methods.push(quote! {
                    #[allow(non_snake_case)]
                    #[inline]
                    pub const fn #new_path_method_ident(self, name: &'static str) -> ErrPath {
                        ErrPath { parent: self, name, flag: #flag_value }
                    }
                });
            }

            quote! {
                impl ErrPathParent {
                    #(#new_path_methods)*
                }
            }
        }

        _ => {
            quote! {}
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn enum_digits(input: TokenStream) -> TokenStream {
    enum_digits::enum_digits(input)
}
