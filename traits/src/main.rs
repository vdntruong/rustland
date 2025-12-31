mod basket; // or mod basket; 'crate' here is optional, represent for root of crate
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new("hi there".to_string());
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hello")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string(&mut b1, "hello".to_string());
    add_string(&mut s1, "world".to_string());
}
