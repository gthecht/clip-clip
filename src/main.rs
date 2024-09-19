use axum::serve;
use server::create_router;
mod leftover;
mod server;
mod types;

#[tokio::main]
async fn main() {
    let app = create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind to port 3000 failed");
    serve(listener, app)
        .await
        .expect("failed to serve app on listener");
}
