pub trait AnimalEating {
    fn eat(&self);
}
pub trait AnimalSound {
    fn make_sound(&self);
}
// Animal is a supertrait
pub trait Animal: AnimalEating + AnimalSound {}

pub struct Dog {}
pub struct Cat {}
pub struct Bear {}

impl AnimalEating for Dog {
    fn eat(&self) {
        println!("Dog eat!");
    }
}
impl AnimalSound for Dog {
    fn make_sound(&self) {
        println!("I'm a dog!");
    }
}
// we still need to implement Animal trait for Dog or Cat or Bear
impl Animal for Dog {}

impl AnimalEating for Cat {
    fn eat(&self) {
        println!("Cat eat!");
    }
}
impl AnimalSound for Cat {
    fn make_sound(&self) {
        println!("I'm a cat!");
    }
}
impl Animal for Cat {}

impl AnimalEating for Bear {
    fn eat(&self) {
        println!("Bear eat!");
    }
}
impl AnimalSound for Bear {
    fn make_sound(&self) {
        println!("I'm a bear!");
    }
}
impl Animal for Bear {}
