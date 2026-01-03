/*

HashSet<T>

HashSet is a collection of unique values
HashSet is implemented using a hash table

https://doc.rust-lang.org/std/collections/struct.HashSet.html
*/

use std::collections::HashSet;

pub fn test() {
    // let _: HashSet<&str> = HashSet::new();

    let mut planet_list: HashSet<&str> = HashSet::from(["Mercury", "Venus", "Earth"]);
    print_planets(&planet_list);

    let planet_list_more = HashSet::from(["Earth", "Mars", "Jupiter"]);
    print_planets(&planet_list_more);

    // difference
    let planet_diff = planet_list.difference(&planet_list_more);
    println!("----------------");
    println!("planet diff {:#?}", planet_diff);
    // symmetric difference, like join outer
    let planet_symdiff = planet_list.symmetric_difference(&planet_list_more);
    println!("----------------");
    println!("planet sym diff {:#?}", planet_symdiff);

    (&mut planet_list).insert("Saturn".as_ref());
    (&mut planet_list).insert("Uranus".as_ref());
    (&mut planet_list).insert("Pluto".as_ref());
    print_planets(&planet_list);
}

fn print_planets(planet_list: &HashSet<&str>) {
    println!("--------------");
    println!("planets {:?}", planet_list);
}