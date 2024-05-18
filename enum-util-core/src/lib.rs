#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{ItemEnum, parse2, Error, Expr, GenericParam, token, ExprLit, LitInt};
use syn::Lit::Int;
use syn::punctuated::Punctuated;

pub fn variant_values_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    variant_values_internal(_args, item).unwrap_or_else(|e| e.to_compile_error())
}

fn variant_values_internal(_args: TokenStream, item: TokenStream) -> Result<TokenStream, Error> {
    let mut enum_item = parse2::<ItemEnum>(item.clone())?;

    let variants = &mut enum_item.variants;
    let len = variants.len();

    for (i, variant) in variants.iter_mut().enumerate() {
        variant.discriminant = Some((token::Eq::default(), Expr::Lit(ExprLit { attrs: vec![], lit: Int(LitInt::new(&format!("{i}"), Span::call_site())) })))
    }

    let name = &enum_item.ident;
    let generics = &enum_item.generics;
    let mut generics_cleaned = enum_item.generics.clone();
    let mut cleaned_params = Punctuated::new();
    for param in generics_cleaned.params.iter_mut() {
        let new_param = match param {
            GenericParam::Type(t) => {
                t.bounds = Punctuated::new();
                GenericParam::Type(t.clone())
            },
            GenericParam::Const(c) => GenericParam::Type(TypeParam {
                attrs: vec![],
                ident: c.ident.clone(),
                colon_token: None,
                bounds: Default::default(),
                eq_token: None,
                default: None,
            }),
            GenericParam::Lifetime(l) => GenericParam::Lifetime(l.clone()),
            _ => continue,
        };

        cleaned_params.push_value(new_param);
        cleaned_params.push_punct(Comma::default())
    }

    generics_cleaned.params = cleaned_params;

    let where_clause = &generics.where_clause;

    let impls = quote! {
        impl #generics #name #generics_cleaned #where_clause {
            const VARIANT_COUNT: usize = #len;
            const fn discriminant(&self) -> usize {
                unsafe { *(self as *const Self as *const usize) }
            }
        }
    };

    let mut tokenstream = enum_item.to_token_stream();
    tokenstream.append_all(impls);
    Ok(tokenstream)
}
