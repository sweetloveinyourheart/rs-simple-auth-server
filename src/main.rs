use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello world" }));

    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("Server is running on port 9000");

    axum::serve(listener, app).await.unwrap();
}
