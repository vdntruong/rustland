use std::env;

pub fn get_db_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_port() -> String {
    env::var("HTTP_PORT").unwrap_or("8080".to_string())
}

pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
pub const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
pub const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
