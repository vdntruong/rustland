pub fn add_string(a: String, b: String) -> String {
    format!("{}{}", a, b)
}

#[cfg(test)]
mod tests {
    use super::add_string;

    #[test]
    fn it_works() {
        let result = add_string("hel".to_string(), "lo".to_string());
        assert_eq!(result, "hello".to_string());
    }
}