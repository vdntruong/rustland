// Rust also allow use to test func via comments
// cargo test --doc
/// ```
/// let full_name = trevounitest::get_full_name("pete", "vo");
/// assert!(full_name == "petevo")
/// ```
pub fn get_full_name(first: &str, last: &str) -> String {
    if first.contains(&['!', '@', '#', '$', '%', '^', '&', '*', '(', ')']) {
        panic!("first cannot contain special characters");
    }
    let mut result = "".to_string();
    result.push_str(first);
    result.push_str(last);
    result.to_string()
}