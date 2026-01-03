// https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html - Match patterns

pub fn test() {
    let my_age: u8 = 30;

    // the first condition check out
    match my_age {
        0|1|2|3|4|5 => println!("You such a baby"),
        5..20 => {
            println!("The value of my age is up to 20");
        }
        30 => {
            println!("The value of my age is 30");
        }
        30.. => {
            println!("The value of my age is out of 30");
        }
        // matches are exhaustive, so dev have to handle all possible cases
        _ => println!("Hi, my age is NOT 30")
    }
}

pub fn test_array() {
    let prices: [u32; 3] = [50_000, 90_000, 120_000];

    // [0..1] -> 50_000
    // [0..=1] -> 50_000 and 90_000
    match prices[0..=1] {
        [30_000, 50_000] => println!("You have some reasonably priced cards"),
        [50_000, 90_000, ..] => println!("You hae a variety of cards!"),
        _ => println!("you don't have any reasonably priced cards!")
    }
}