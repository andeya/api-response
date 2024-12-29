use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{Ident, LitInt, Token, parse_macro_input};

#[proc_macro]
pub fn enum_segment(input: TokenStream) -> TokenStream {
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
