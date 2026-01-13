mod vehicle;

use axum::{routing::get, Router};
use vehicle::{handle_get_vehicle, handle_post_vehicle};

#[tokio::main]
async fn main() {
    // Load configurations
    let host = "127.0.0.1";
    let port = "7878";

    // Create the Router
    let app = Router::new()
        .route("/vehicles", get(handle_get_vehicle).post(handle_post_vehicle));

    // Create the listener
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await.unwrap();

    // Launch the web server
    println!("Listening on http://{host}:{port}");
    axum::serve(listener, app).await.unwrap();
}
