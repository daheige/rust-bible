use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
mod router;

// 采用tokio async框架运行axum
#[tokio::main]
async fn main() {
    println!("rs-api start");
    let app = Router::new()
        .route("/", get(router::home))
        .route("/users", post(router::create_user));

    let socket: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    let address = SocketAddr::from((socket.ip(), socket.port()));
    println!("http service run:{:?}", address);

    // 创建http service
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
