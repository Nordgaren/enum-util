#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{ItemEnum, parse2, Error, token, Expr, ExprLit, LitInt};
use syn::Lit::Int;

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

    let impls = quote! {
        impl #name {
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
