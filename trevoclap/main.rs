use clap::{Arg, Command, command};
// ArgGroup

// cargo run -q -- --help
// cargo run -q -- -h

// cargo run -q -- --concho meow

// cargo run -q -- personal
// cargo run -q -- personal -h
// cargo run -q -- personal --firstname Pete --lastname Vo
// cargo run -q -- personal -f Pete -l Vo
// cargo run -q -- personal --firstname Pete --age 28
// cargo run -q -- personal --firstname Pete --lastname Vo --age 28  ==> Conflicted

// cargo run -q -- public
// cargo run -q -- public -h
// cargo run -q -- public --state HUE --city HUECITY

fn main() {
    let match_result = command!()
        .about(clap::crate_description!())
        .subcommand(
            Command::new("personal")
                .about("Personal command")
                .arg(
                    Arg::new("firstname")
                        .required(true)
                        .short('f')
                        .long("firstname")
                        .aliases(["firstname", "fname", "first-name", "fn"])
                        .value_name("PETE")
                        .help("First name to print"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("lastname")
                        .value_name("VO")
                        .help("Last name to print")
                        .conflicts_with("age"),
                )
                .arg(
                    Arg::new("age")
                        .long("age")
                        .value_name("AGE")
                        .help("Age to print")
                        .conflicts_with("lastname"),
                ),
        )
        .subcommand(
            Command::new("public")
                .about("Public command")
                .arg(
                    Arg::new("state")
                        .required(true)
                        .long("state")
                        .short('s')
                        .help("State to use"),
                )
                .arg(
                    Arg::new("city")
                        .short('c')
                        .long("city")
                        .help("City to use")
                ),
        )
        .arg(
            Arg::new("concho")
                .short('c')
                .long("concho")
                .value_name("DO")
                .help("Con cho con")
        )
        // .group(ArgGroup::new("personal").args(&["firstname", "lastname", "age"]))
        // .group(ArgGroup::new("public"))
        .get_matches();

    match match_result.subcommand_matches("personal") {
        Some(personal_args) => {
            println!("👤  👤  Personal command");

            println!("first name {:#?}", personal_args.get_one::<String>("firstname").unwrap());

            let lastname = personal_args.get_one::<String>("lastname");
            if lastname.is_some() {
                println!("Last name {:#?}", lastname.unwrap());
            }

            let age = personal_args.get_one::<String>("age");
            if age.is_some() {
                println!("Age {:#?}", age.unwrap());
            }
        }
        None => {}
    }

    match match_result.subcommand_matches("public") {
        Some(public_args) => {
            println!("🌍  🌍  Public command");

            println!("State {:#?}", public_args.get_one::<String>("state").unwrap());

            let city = public_args.get_one::<String>("city");
            if city.is_some() {
                println!("City {:#?}", city.unwrap());
            }
        }
        None => {}
    }

    let concho = match_result.get_one::<String>("concho");
    if concho.is_some() {
        println!("🐕  🐶  Concho command");


        println!("concho {:#?}", concho.unwrap());
    }
}
