#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;


pub fn attribute_macro_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}
