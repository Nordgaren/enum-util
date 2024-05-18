#![cfg(test)]

use crate::variant_values_impl;
use quote::quote;

#[test]
fn test() {
    let after = variant_values_impl(quote!(), quote!());
    assert_ne!(after.to_string(), "");
}
