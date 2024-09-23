use std::env;

use axum::serve;
use server::create_router;
mod leftover;
mod server;
mod types;

#[tokio::main]
async fn main() {
    let app = create_router();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&host)
        .await
        .expect(&format!("bind to port {} failed", port));
    println!("app listening on {}", host);
    serve(listener, app)
        .await
        .expect("failed to serve app on listener");
}
