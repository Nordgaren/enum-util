#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt, format_ident};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Lit::Int;
use syn::{parse2, token, Error, Expr, ExprLit, GenericParam, ItemEnum, LitInt, TypeParam, Generics, Attribute, AttrStyle, Path, PathSegment, Fields};
use syn::parse::Parser;
use syn::spanned::Spanned;

pub fn variant_values_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    variant_values_internal(_args, item).unwrap_or_else(|e| e.to_compile_error())
}

fn variant_values_internal(_args: TokenStream, item: TokenStream) -> Result<TokenStream, Error> {
    let mut enum_item = parse2::<ItemEnum>(item.clone())?;
    let repr = enforce_repr_inttype(&mut enum_item)?;

    let variants = &mut enum_item.variants;
    let len = variants.len();

    for (i, variant) in variants.iter_mut().enumerate() {
        variant.discriminant = Some((
            token::Eq::default(),
            Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Int(LitInt::new(&format!("{i}"), Span::call_site())),
            }),
        ))
    }

    let name = &enum_item.ident;
    let generics = &enum_item.generics;
    let mut generics_cleaned = get_cleaned_generics(&enum_item);
    let where_clause = &generics.where_clause;

    let mut vals = Punctuated::new();
    for variant in &enum_item.variants {
        let field = match variant.fields {
            Fields::Named(_) => quote! {(_)},
            Fields::Unnamed(_) => quote! {(_)},
            Fields::Unit => quote! {},
        };
        let v_ident = &variant.ident;
        let v_disc = variant.discriminant.clone().unwrap().1;
        vals.push_value(quote!{ #name::#v_ident #field => #v_disc});
        vals.push_punct(Comma::default());
    }

    let from = quote! {
         impl #generics From<&#name #generics_cleaned> for #repr #where_clause {
            fn from(value: &#name #generics_cleaned) -> Self {
                value.discriminant()
            }
        }
        impl #generics From<#name #generics_cleaned> for #repr #where_clause {
            fn from(value: #name #generics_cleaned) -> Self {
                Self::from(&value)
            }
        }
    };

    eprintln!("{}", from.to_string());

    let impls = quote! {
        impl #generics #name #generics_cleaned #where_clause {
            const VARIANT_COUNT: usize = #len;
            const fn discriminant(&self) -> #repr {
                match self {
                    #vals
                }
            }
        }
    };

    let mut tokenstream = enum_item.to_token_stream();
    tokenstream.append_all([impls, from]);
    Ok(tokenstream)
}

fn enforce_repr_inttype(input: &mut ItemEnum) -> Result<Ident, Error> {
    let len = input.variants.len();
    let attrs = &input.attrs;
    for attr in attrs {
        if attr.path.to_token_stream().to_string() == "repr" {
            let tokens = attr.tokens.to_string();
            if !tokens.starts_with("(i") && !tokens.starts_with("(u") {
                return Err(Error::new(
                    input.span(),
                    format!(
                        "Found `#[repr{}]` attribute. Type must have `#[repr(inttype)]` attribute.",
                        tokens
                    )));
            }

            return Ok(format_ident!("{}", tokens.trim_matches(&['(', ')'])));
        }
    }

    let tokens = if len <= u8::MAX as usize {
        quote!((u8))
    } else if len <= u16::MAX as usize {
        quote!((u16))
    } else if len <= u32::MAX as usize {
        quote!((u32))
    } else if len <= u64::MAX as usize {
        quote!((u64))
    } else if len <= u128::MAX as usize {
        quote!((u128))
    } else {
        quote!((usize))
    };

    let repr = tokens.clone();

    input.attrs.push(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        path: Path {
            leading_colon: None,
            segments: Punctuated::from_iter([PathSegment {
                ident: format_ident!("repr"),
                arguments: Default::default(),
            }]),
        },
        tokens,
    });

    Ok(format_ident!("{}", repr.to_string().trim_matches(&['(', ')'])))
}


fn get_cleaned_generics(enum_item: &ItemEnum) -> Generics {
    let mut generics_cleaned = enum_item.generics.clone();
    let mut cleaned_params = Punctuated::new();
    for param in generics_cleaned.params.iter_mut() {
        let new_param = match param {
            GenericParam::Type(t) => {
                t.bounds = Punctuated::new();
                GenericParam::Type(t.clone())
            }
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

    generics_cleaned
}
