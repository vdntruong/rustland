#![cfg(test)]

/*
'#' mean that apply the attribute to the following item
'!' mean that apply the attribute to the entire file
 */

#[test] // to declare a test function, so that we can run: cargo test
fn test_get_full_name() {
    // use super::*;
    // use super::get_full_name;
    let result = trevounitest::get_full_name("pete", "vo");
    assert_eq!("petevo", result);
}

#[test]
fn test_get_full_name_with_space() {
    let result = trevounitest::get_full_name("pete", " vo");
    assert_eq!("pete vo", result);
}

#[test]
#[should_panic] // to expect a panic
fn test_get_full_name_special_chars() {
    _ = trevounitest::get_full_name("Tr&pet!e", "vo");
}
