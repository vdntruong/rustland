mod constant;
mod model;
mod handler;

use constant::*;
use model::User;
use handler::*;
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[macro_use]
extern crate serde_derive;

// set database function
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(&get_db_url(), NoTls)?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL
        )",
        &[],
    )?;

    println!("database created successfully");
    Ok(())
}

// get id function
fn get_id_from_request(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// deserialize user from request body with the id
fn deserialize_user_from_request(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

fn main() {
    if let Err(e) = set_database() {
        eprintln!("Failed to set database: {}", e);
        return;
    }

    let port = get_port();
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening at {}", &addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// handle connection function
fn handle_connection(mut stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(&String::from_utf8_lossy(&buffer[..size]));

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post(r),
                r if r.starts_with("GET /users/") => handle_get_by_id(r),
                r if r.starts_with("GET /users") => handle_get_all(r),
                r if r.starts_with("PUT /users/") => handle_put(r),
                r if r.starts_with("DELETE /users/") => handle_delete(r),

                _ => (NOT_FOUND.to_string(), "NOT FOUND".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return;
        }
    }
}
