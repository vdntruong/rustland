use std::env;

fn main() {
    // cargo run -q hello

    let mut my_args = env::args().collect::<Vec<String>>();

    println!("Args {:?}", my_args);
    println!("Args len: {}", my_args.len());

    if my_args.len() < 3 {
        eprintln!("Please pass in 2 args");
        return;
    }

    let name = my_args.get(1).unwrap();
    println!("Name: {:?}", name);

    let age = my_args.get(2).unwrap().parse::<u8>();
    if age.is_err() {
        eprintln!("Age - Please pass in the age as second argument");
        return;
    }
    println!("Age: {:?}", age.unwrap());
}
