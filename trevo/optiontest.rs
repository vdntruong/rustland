fn option_none() -> Option<u8> {
    let mut opt: Option<u8> = None;

    opt = Some(10);

    opt
}

// `CharacterType` does not implement `Display` (required by `{}`): E0277
// we can use #[derive(Debug)] for print(":#?") or implement the to_string associate fn for Display
enum CharacterType {
    Archer,
    Warrior,
    Mage
}

impl CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer",
            CharacterType::Warrior => "Warrior",
            CharacterType::Mage =>"Mage"
        }.to_string()
    }
}

fn enum_option() -> Option<CharacterType> {
    let mut opt: Option<CharacterType> = None;
    opt = Some(CharacterType::Archer);
    opt
}

pub fn test() {
    let result = option_none();

    println!("result is {0}", result.unwrap());
    // if the result is None, we'll get an error like:
    // thread ... panicked at optiontest.rs:8:38:
    // called `Option::unwrap()` on a `None` value

    println!("result is {:?}", result);

    let character_type = enum_option();

    if character_type.is_some() { // we can use is_some or is_none to check
        println!("character {:?}", character_type.unwrap().to_string())
    } else if character_type.is_none() {
        println!("None")
    }
}