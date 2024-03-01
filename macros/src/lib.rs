use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{LitByte, LitByteStr, LitChar, LitStr};

#[proc_macro]
pub fn from_str(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as Option<LitStr>);
    expand_chars(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn from_bytes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as Option<LitByteStr>);
    expand_bytes(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

enum Chars {
    Cons(LitChar, Box<Self>),
    Nil,
}

impl ToTokens for Chars {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Chars::Cons(c, rest) => quote!(::typeslice::types::Char<#c, #rest>),
            Chars::Nil => quote!(::typeslice::types::CharNil),
        })
    }
}

fn expand_chars(lit: Option<LitStr>) -> syn::Result<TokenStream> {
    let root = match lit {
        Some(it) => it.value().chars().rev().fold(Chars::Nil, |acc, el| {
            Chars::Cons(LitChar::new(el, it.span()), Box::new(acc))
        }),
        None => Chars::Nil,
    };
    Ok(root.into_token_stream())
}

enum Bytes {
    Cons(LitByte, Box<Self>),
    Nil,
}

impl ToTokens for Bytes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Bytes::Cons(b, rest) => quote!(::typeslice::types::U8<#b, #rest>),
            Bytes::Nil => quote!(::typeslice::types::U8Nil),
        })
    }
}

fn expand_bytes(lit: Option<LitByteStr>) -> syn::Result<TokenStream> {
    let root = match lit {
        Some(it) => it.value().into_iter().rev().fold(Bytes::Nil, |acc, el| {
            Bytes::Cons(LitByte::new(el, it.span()), Box::new(acc))
        }),
        None => Bytes::Nil,
    };
    Ok(root.into_token_stream())
}
