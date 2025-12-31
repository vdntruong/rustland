use postgres::{Client, NoTls};
use crate::constant::{get_db_url, INTERNAL_SERVER_ERROR, NOT_FOUND, OK_RESPONSE};
use crate::{deserialize_user_from_request, get_id_from_request};
use crate::model::User;

// handle post to create new user
pub fn handle_post(request: &str) -> (String, String) {
    println!("POST /users");
    match (
        deserialize_user_from_request(&request),
        Client::connect(&get_db_url(), NoTls),
    ) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
                    &[&user.name, &user.email],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        (Err(e), _) => {
            eprintln!("Failed to deserialize user payload: {}", e.to_string());
            (INTERNAL_SERVER_ERROR.to_string(), "BAD REQUEST".to_string())
        },
        (_, Err(e)) => {
            eprintln!("Failed to connect to database: {}", e.to_string());
            (INTERNAL_SERVER_ERROR.to_string(), "Internal Server Error".to_string())
        },
    }
}

// handle get by id to get user by id
pub fn handle_get_by_id(request: &str) -> (String, String) {
    println!("GET /users/{id}", id = get_id_from_request(request));
    match (
        get_id_from_request(&request).parse::<i32>(),
        Client::connect(&get_db_url(), NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                Err(_) => (NOT_FOUND.to_string(), "Not Found".to_string()),
            }
        }
        _ => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Internal Server Error".to_string(),
        ),
    }
}

// handle get all to get all users
pub fn handle_get_all(_: &str) -> (String, String) {
    println!("GET /users");
    match Client::connect(&get_db_url(), NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }

            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&users).unwrap(),
            )
        }
        _ => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Internal Server Error".to_string(),
        ),
    }
}

// handle put to update user
pub fn handle_put(request: &str) -> (String, String) {
    println!("PUT /users/{id}", id = get_id_from_request(request));
    match (
        get_id_from_request(request).parse::<i32>(),
        deserialize_user_from_request(request),
        Client::connect(&get_db_url(), NoTls),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id],
                )
                .unwrap();

            (
                OK_RESPONSE.to_string(),
                "User updated".to_string(),
            )
        }
        _ => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Internal Server Error".to_string(),
        ),
    }
}

// handle delete to delete user
pub fn handle_delete(request: &str) -> (String, String) {
    println!("DELETE /users/{id}", id = get_id_from_request(request));
    match (
        get_id_from_request(request).parse::<i32>(),
        Client::connect(&get_db_url(), NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            let row_affected = client
                .execute("DELETE FROM users WHERE id = $1", &[&id])
                .unwrap();
            if row_affected == 0 {
                return (NOT_FOUND.to_string(), "Not Found".to_string());
            }

            (
                OK_RESPONSE.to_string(),
                "User deleted".to_string(),
            )
        }
        _ => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Internal Server Error".to_string(),
        ),
    }
}
