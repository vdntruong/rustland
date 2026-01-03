#[derive(Debug)]
struct Person {
    name: String,
}

pub fn test_closures() {
    let pri: fn() = || println!("This is a closure");
    pri();

    let add: fn(i8, i32) -> i32 = |x: i8, y: i32| x as i32 + y;
    let result: i32 = add(127, 128);

    let prir = |x: i32| println!("This is the result from `add` + {}: {}", x, result + x);
    prir(32);

    let mut person: Person = Person {
        name: String::from("John"),
    };
    // 'mut' keywork for the closure itself is required for the variable to be mutable
    let mut change_person = |new_name: &str| person.name = new_name.to_string();
    change_person("Pete");
    println!("The person {:#?}", person);

    // if we try to change the person's name again, we will get an error
    // change_person("David");
    // 'cannot borrow `person` as immutable because it is also borrowed as mutable [E0502]'
    // this because the closure is borrowing the variable mutably

    // to fix this, we can use a reference to the variable
    // or wait for the change completion, like:
    //
    // change_person("Pete");
    // change_person("David");
    // println!("The person {:#?}", person);
}
