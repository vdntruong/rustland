mod animal;

use animal::{AnimalEating, AnimalSound};
use animal::{Bear, Cat, Dog};
use crate::animal::Animal;

fn make_some_noise<T: AnimalSound>(p: T) {
    p.make_sound();
}

fn make_some_noise_with_dyn(a: &dyn AnimalSound) {
    a.make_sound();
}

fn eat_some_food(a: &dyn AnimalEating) {
    a.eat();
}

fn do_it(a: &dyn Animal) {
    println!("Let's do it!");
    a.make_sound();
    a.eat();
}

// dynamic dispatch in return function
fn get_eating_animal() -> Box<dyn AnimalEating> {
    let bear: Bear = Bear {};
    Box::from(bear)
}

fn get_an_animal() -> Box<dyn Animal> {
    let animal = Dog{};
    Box::from(animal)
}

fn main() {
    println!();
    // normal trait call

    let dog01: Dog = Dog {};
    make_some_noise(dog01);

    let cat01: Cat = Cat {};
    make_some_noise(cat01);

    let bear01: Bear = Bear {};
    make_some_noise(bear01);

    println!();
    // dynamic trait call/dispatch

    // dyn_ans is a dynamic reference to an AnimalSound trait object (the implementation if the trait ~ Dog)
    // so the dyn_ans variable can also point to a Cat or Bear
    let mut dyn_ans: &dyn AnimalSound = &Dog {};
    make_some_noise_with_dyn(dyn_ans);

    dyn_ans = &Cat {};
    make_some_noise_with_dyn(dyn_ans);

    dyn_ans = &Bear {};
    make_some_noise_with_dyn(dyn_ans);

    let dyn_ans: &dyn AnimalEating = &Dog {};
    eat_some_food(dyn_ans);

    println!();
    // dynamic dispatch in return

    let dyn_some_eating_animal = get_eating_animal();
    dyn_some_eating_animal.eat();

    let some_pet = get_an_animal();
    some_pet.eat();
    some_pet.make_sound();

    println!();
    do_it(some_pet.as_ref());
}
