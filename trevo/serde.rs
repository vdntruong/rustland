/*

serde crate
- ser stands for serialisation
- de stands for deserialisation

serde supports to convert between data structures and other formats like JSON, YAML, TOML, etc.

serde_json is a crate that provides a JSON implementation for serde.
serde_yaml is a crate that provides a YAML implementation for serde.
...etc

https://docs.rs/serde/latest/serde/
https://crates.io/crates/serde

Install:
  cargo add serde --features derive
  cargo add serde_json
 */

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

// https://serde.rs/attributes.html
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // serde --features derive
struct Person {
    first_name: String,
    age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // serde --features derive
#[serde(deny_unknown_fields)] // serde --features derive
struct Pet {
    name: String,
    #[serde(rename = "birthday")] // change field name in JSON to birthday
    year_born: i32,
    owner: Person,
}

pub fn test() {
    // Serialize
    println!();

    let person01: Person = Person {
        first_name: "John Doe".to_string(),
        age: 30,
    };

    let person_json = to_string_pretty(&person01).unwrap();
    println!("person JSON: {}", person_json);
    // {
    //   "firstName": "John Doe",
    //   "age": 30
    // }

    let pet01: Pet = Pet {
        name: "Buddy".to_string(),
        year_born: 2020,
        owner: person01,
    };

    let pet_json = to_string(&pet01).unwrap();
    println!("pet JSON: {}", pet_json);
    // {"name":"Buddy","birthday":2020,"owner":{"firstName":"John Doe","age":30}}

    // Deserialize
    println!();

    // "additionalfield":"something", any field not defined in the struct will cause error by attribute #[serde(deny_unknown_fields)]
    let json_string =
        r#"{"name":"Buddy","birthday":2020,"owner":{"firstName":"John Doe","age":30}}"#;
    let pet_deser = from_str::<Pet>(&json_string);
    if pet_deser.is_err() {
        println!("Error: {}", pet_deser.unwrap_err());
    } else {
        println!("pet deser: {:#?}", pet_deser.unwrap());
    }
}
