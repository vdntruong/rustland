/*
 Traits inside of Rust only allow you to define method signatures, similar to interface in Go.
*/

#[derive(Debug)]
struct Dog {}

impl Dog {
    fn new() -> Self {
        Dog {}
    }
}

#[derive(Debug)]
struct Cat {}

impl Cat {
    fn new() -> Self {
        Cat {}
    }
}

struct Bear {}

impl Bear {
    fn new() -> Self {
        Bear {}
    }
}

trait Animal {
    fn noise(&self);
}

impl Animal for Dog {
    fn noise(&self) -> () {
        println!("I am a dog");
    }
}

impl Animal for Cat {
    fn noise(&self) -> () {
        println!("I am a cat");
    }
}

impl Animal for Bear {
    fn noise(&self) -> () {
        println!("I am a bear");
    }
}

trait NotDangerous {}

impl NotDangerous for Dog {}

impl NotDangerous for Cat {}

// the syntax <PetType: Animal> is called trait bounds (trait boundaries)
// the <PetType: ...>, PetType is the generic type or generic annotation
//
// another way to write this is:
// struct Person<PetType> where PetType: Animal + NotDangerous {}
#[derive(Debug)]
struct Person<PetType: Animal + NotDangerous> {
    // this mean the PetType arg have to implement both
    name: String,
    pet: PetType,
}

impl<PetType: Animal + NotDangerous> Person<PetType> {
    fn new(name: String, pet: PetType) -> Self {
        Person { name, pet }
    }
}

pub fn test() {
    let pet1: Dog = Dog::new();
    let pet2: Cat = Cat::new();
    let pet3: Bear = Bear::new();

    let pete: Person<Dog> = Person::new("Pete".to_string(), pet1);
    // Person {
    //     name: "Pete".to_string(),
    //     pet: pet1,
    // };

    println!("pete name is {:?}", pete);
    pete.pet.noise();
}
