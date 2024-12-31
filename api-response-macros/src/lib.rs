use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{Ident, LitInt, Token, parse_macro_input};

#[proc_macro]
pub fn enum_digits(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as EnumSegmentInput);

    let name = input.name;
    let repr_type = input.repr_type;
    let start = input.start;
    let end = input.end;
    let prefix = input.prefix;
    let width = input.width;

    let mut variants = Vec::new();

    for i in start..=end {
        let variant_name = format!("{prefix}{:0width$}", i, width = width as usize);
        let variant_ident = Ident::new(&variant_name, name.span());
        let variant_value = Literal::i32_unsuffixed(i);

        variants.push(quote! {
            #[allow(non_camel_case_types)]
            #variant_ident = #variant_value,
        });
    }

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        #[derive(num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
        #[repr(#repr_type)]
        #[non_exhaustive]
        pub enum #name {
            #(#variants)*
        }
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:0>width$}", #repr_type::from(*self), width = #width as usize)
            }
        }
    };

    TokenStream::from(expanded)
}

struct EnumSegmentInput {
    name: Ident,
    repr_type: Ident,
    start: i32,
    end: i32,
    prefix: String,
    width: u32,
}

impl syn::parse::Parse for EnumSegmentInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let repr_type: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let start: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;

        let end: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;

        let prefix: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let width: LitInt = input.parse()?;

        Ok(EnumSegmentInput {
            name,
            repr_type,
            start: start.base10_parse::<i32>()?,
            end: end.base10_parse::<i32>()?,
            prefix: prefix.to_string(),
            width: width.base10_parse::<u32>()?,
        })
    }
}

#[proc_macro]
pub fn err_path_types(_input: TokenStream) -> TokenStream {
    let mut new_root_funcs = Vec::new();
    let mut new_parent_methods = Vec::new();
    let mut new_path_methods = Vec::new();

    for i in 0..=99u8 {
        let flag_value = Literal::u8_unsuffixed(i);
        let new_root_func_ident = Ident::new(&format!("X{i:02}"), Span::call_site());
        let new_parent_method_ident = Ident::new(&format!("Y{i:02}"), Span::call_site());
        let new_path_method_ident = Ident::new(&format!("Z{i:02}"), Span::call_site());

        new_root_funcs.push(quote! {
            #[allow(non_snake_case)]
            #[inline]
            pub const fn #new_root_func_ident(name: &'static str) -> ErrPathRoot {
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
        new_path_methods.push(quote! {
            #[allow(non_snake_case)]
            #[inline]
            pub const fn #new_path_method_ident(self, name: &'static str) -> ErrPath {
                ErrPath { parent: self, name, flag: #flag_value }
            }
        });
    }

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, getset2::Getset2)]
        #[getset2(get_copy(pub, const))]
        #[non_exhaustive]
        pub struct ErrPathRoot {
            name: &'static str,
            flag: u8,
        }
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, getset2::Getset2)]
        #[getset2(get_copy(pub, const))]
        #[non_exhaustive]
        pub struct ErrPathParent {
            root: ErrPathRoot,
            name: &'static str,
            flag: u8,
        }
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, getset2::Getset2)]
        #[getset2(get_copy(pub, const))]
        #[non_exhaustive]
        pub struct ErrPath {
            parent: ErrPathParent,
            name: &'static str,
            flag: u8,
        }
        impl std::fmt::Display for ErrPathRoot {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "X{:02}({})", self.flag, self.name)
            }
        }
        impl std::fmt::Display for ErrPathParent {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}/Y{:02}({})", self.root, self.flag, self.name)
            }
        }
        impl std::fmt::Display for ErrPath {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}/Z{:02}({})", self.parent, self.flag, self.name)
            }
        }

        #(#new_root_funcs)*

        impl ErrPathRoot {
            #[inline]
            pub const fn default() -> Self {
                Self {
                    name: "",
                    flag: 0,
                }
            }
            #[inline]
            pub fn path(&self) -> String {
                self.to_string()
            }
            #[inline]
            pub const fn path_flag(&self) -> i32 {
                self.flag as i32
            }
            #(#new_parent_methods)*
        }

        impl ErrPathParent {
            #[inline]
            pub const fn default() -> Self {
                Self {
                    root: ErrPathRoot::default(),
                    name: "",
                    flag: 0,
                }
            }
            #[inline]
            pub fn path(&self) -> String {
                self.to_string()
            }
            #[inline]
            pub const fn path_flag(&self) -> i32 {
                (self.root.path_flag() * 100) + self.flag as i32
            }
            #(#new_path_methods)*
        }

        impl ErrPath {
            #[inline]
            pub const fn default() -> Self {
                Self {
                    parent: ErrPathParent::default(),
                    name: "",
                    flag: 0,
                }
            }
            #[inline]
            pub fn path(&self) -> String {
                self.to_string()
            }
            #[inline]
            pub const fn path_flag(&self) -> i32 {
                (self.parent.path_flag() * 100) + self.flag as i32
            }
        }
    };

    TokenStream::from(expanded)
}
