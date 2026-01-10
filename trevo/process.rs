/*
process crate is a cross platform process management library

https://doc.rust-lang.org/std/process/index.html
 */

use std::process::Command;

pub fn test() {
    let p1_result = Command::new("which").arg("python3").output();
    if p1_result.is_ok() {
        let result = p1_result.ok().unwrap();
        println!("was execution successful? {}", result.status.success());

        if !result.status.success() {
            println!("error occurred: {}", result.status.code().unwrap());
        }
    }
}
