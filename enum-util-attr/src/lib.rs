#![doc = include_str!("../README.md")]

use enum_util_core::variant_values_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn variant_values(_args: TokenStream, item: TokenStream) -> TokenStream {
    variant_values_impl(_args.into(), item.into()).into()
}
