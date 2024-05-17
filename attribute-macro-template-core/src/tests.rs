#![cfg(test)]

use crate::attribute_macro_impl;
use quote::quote;

#[test]
fn test() {
    let after = attribute_macro_impl(quote!(), quote!());
    assert_ne!(
        after.to_string(),
        ""
    );
}
