#![doc = include_str!("../README.md")]

use attribute_macro_template_core::attribute_macro_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn attribute_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute_macro_impl(args.into(), input.into()).into()
}