use mylib01;

fn main() {
    println!("{}", mylib01::mathops::add(1, 2));
    println!("{}", mylib01::strops::util::add_string("hello".to_string(), " world".to_string()));
    println!("Hello, world!");
}
