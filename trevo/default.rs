/*

This crate shows how to use default trait and custom primitive types

default trait is like zero value in Go
https://doc.rust-lang.org/std/default/trait.Default.html
 */

// Custom primitive type
#[derive(Debug)]
struct FirstName(String);

impl Default for FirstName {
    fn default() -> Self {
        Self(String::from("Pete"))
    }
}

#[derive(Debug)]
// #[derive(Default)] we don't want fields as empty so we implement it manually
struct Person {
    first_name: FirstName,
    last_name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            first_name: FirstName::default(),
            last_name: String::from("Vo"),
            age: 29,
        }
    }
}

pub fn test() {
    println!("Default String: '{}'", String::default());
    println!("Default u8: {}", u8::default());

    let person = Person::default();
    println!("Default person is: {:#?}", person);

    // We can use default() fn to fulfill the missing fields with default values
    let other_person = Person {
        first_name: FirstName("David".to_string()),
        ..Default::default()
    };
    println!("Other person is: {:#?}", other_person)
}
