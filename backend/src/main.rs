use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", routes::user::router());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000)); 
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app)
        .await
        .unwrap();
}