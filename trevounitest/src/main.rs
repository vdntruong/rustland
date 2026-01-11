mod main_test; // reference to the test file

fn main() {
    println!("{}", trevounitest::get_full_name("pete".as_ref(), "vo"));
}

// fn get_full_name(first: &str, last: &str) -> String {
//     if first.contains(&['!', '@', '#', '$', '%', '^', '&', '*', '(', ')']) {
//         panic!("first cannot contain special characters");
//     }
//     let mut result = "".to_string();
//     result.push_str(first);
//     result.push_str(last);
//     result.to_string()
// }

// #[cfg(test)] // to exclude from the build
// mod mytests {
//     #[test] // to declare a test function, so that we can run: cargo test
//     fn test_get_full_name() {
//         // use super::*;
//         // use super::get_full_name;
//         let result = super::get_full_name("pete", "vo");
//         assert_eq!("petevo", result);
//     }

//     #[test]
//     fn test_get_full_name_with_space() {
//         let result = super::get_full_name("pete", " vo");
//         assert_eq!("pete vo", result);
//     }

//     #[test]
//     #[should_panic] // to expect a panic
//     fn test_get_full_name_special_chars() {
//         _ = super::get_full_name("Tr&pet!e", "vo");
//     }
// }
