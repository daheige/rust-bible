use axum::Server;
use std::net::SocketAddr;

mod router;

// 采用tokio async框架运行axum
#[tokio::main]
async fn main() {
    println!("rs-api start");
    let app = router::create_app();

    // 指定运行地址和端口
    let socket: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let address = SocketAddr::from((socket.ip(), socket.port()));
    println!("http service run:{:?}", address);

    // 创建http service
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
