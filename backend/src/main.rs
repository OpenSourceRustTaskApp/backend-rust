use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // ルートパス "/" に GET リクエストが来たときに "Hello, world!" を返す
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // 8000番ポートでサーバーを起動
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000)); 
    let listener = TcpListener::bind(addr).await.unwrap(); // TcpListener を作成
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap(); 
}