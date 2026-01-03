/*

HashMap<K, V>
HashSet<T>

HashMap is a collection of key-value pairs
HashSet is a collection of unique values

https://doc.rust-lang.org/std/collections/struct.HashMap.html

HashMap, and HashSet are implemented using a hash table

They both provide via std::collections
https://doc.rust-lang.org/std/collections/index.html
*/

use std::collections::{HashMap, HashSet};

pub fn test() {
    // let _: HashMap<String, f32> = HashMap::new();
    // let _ = HashMap::<String, f32>::new();

    let mut stock_list: HashMap<String, f32> = HashMap::new();
    println!("Is empty: {:?}", stock_list.is_empty());

    (&mut stock_list).insert("SSI".to_string(), 29.50);
    (&mut stock_list).insert("HPG".to_string(), 60.50);
    (&mut stock_list).insert("VIC".to_string(), 33.30);

    print_stock_list(&stock_list);

    // overriding
    (&mut stock_list).insert("VIC".to_string(), 13.30);
    print_stock_list(&stock_list);

    // insert or nothing
    stock_list.entry("VIC".to_string()).or_insert(0.0);
    print_stock_list(&stock_list); // because we already have VIC present so there is no affect

    // loop through the hash map
    for (key, value) in &stock_list {
        println!("{}: {}", key, value);
    }

    // removing
    (&mut stock_list).remove(&"VIC".to_string());
    print_stock_list(&stock_list);

    // clear
    (&mut stock_list).clear();
    print_stock_list(&stock_list);
}

fn print_stock_list(stock_list: &HashMap<String, f32>) {
    // # petty print
    println!("-----------------");
    println!("{:#?}",           stock_list);
    println!("Len: {:?}",       stock_list.len());
    println!("Is empty: {:?}",  stock_list.is_empty());
}
