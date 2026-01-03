/*

https://doc.rust-lang.org/book/ch13-01-closures.html
https://doc.rust-lang.org/std/thread/fn.scope.html
 */
use std::thread;

struct Person {
    name: String,
}

pub fn test() {
    let age: u8 = 30;
    let pete: Person = Person {
        name: String::from("Pete"),
    };

    // the 'move' keyword is required to move the ownership of the variable 'age' to the thread
    // (required for the thread to take ownership of the variable)
    // let print_age = move || {
    //     println!("My age is {age}");
    //     println!("This is person name {}", pete.name);
    // };
    let print_age = move || {
        println!("This is the child closure");
        println!("My age is {age}");
        println!("This is person name {}", &pete.name);
    };

    // with '_' prefix, we are not interested in the result of the thread
    // and the compiler will not warn us if we don't use the result
    let _result = thread::spawn(print_age).join();
    println!("Finished printing age");

    // println!("My age is {age}");                     // we can print age because closure just copied the value
    // println!("This is person name {}", pete.name);   // but we can not print name because we moved ownership to the thread
}
