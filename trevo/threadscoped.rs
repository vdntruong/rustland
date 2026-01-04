/*

Scope thread is a way to create a new thread and join it to the main thread.

The primary benefit of scope is that it allows threads to safely borrow non-static data from the parent scope without needing to use synchronization primitives like Arc or Mutex.

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

    // with '_' prefix, we are not interested in the result of the thread
    // and the compiler will not warn us if we don't use the result
    // let _result = thread::spawn(print_age).join();
    // println!("Finished printing age");

    // scope

    let print_age2 = || {
        // removed 'move' keyword
        println!("This is the child closure");
        println!("My age is {age}");
        println!("This is person name {}", &pete.name); // using reference to avoid moving ownership (intead of using 'move' to move ownership)
    };
    thread::scope(|s| {
        // we will call scope's spawn method
        s.spawn(print_age2); // scope auto join threads to the main thread
        s.spawn(print_age2);
    });

    println!("This is the main thread");
    println!("My age is {age}"); // we can print age because closure just copied the value
    println!("This is person name {}", pete.name); // but we can not print name because we moved ownership to the thread
}
