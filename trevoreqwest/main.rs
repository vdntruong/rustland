/*

reqwest - 3-party http client in Rust

https://docs.rs/reqwest/latest/reqwest/
 */

use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;

fn main() {
    let client = Client::new();

    let get_result = client.get("https://hyper.rs").send();
    if get_result.is_ok() {
        println!("Response: {:#?}", get_result.ok().unwrap().text().unwrap());
    } else if get_result.is_err() {
        println!("Error: {:#?}", get_result.err().unwrap());
    }

    let post_result = client.post("https://hyper.rs")
        .body("something")
        .header("Content-Type", "application/json")
        .header("X-Requested-With", "authorization")
        .send();
    if post_result.is_ok() {
        println!("Response: {:#?}", post_result.ok().unwrap());
    } else {
        println!("Error: {:#?}", post_result.err().unwrap());
    }

    // redirect
    let redir_policy = Policy::limited(5);
    let client2 = ClientBuilder::new().redirect(redir_policy).build().ok().unwrap();

    /* this old path must respond:
    - Status: 301 - Moved Permanently
    - Header: Location | <new path>
     */
    let get_result2 = client2.get("https://hyper.rs/old-path").send();
    if get_result2.is_ok() {
        println!("Response: {:#?}", get_result2.ok().unwrap());
    }
}
