use axum::{
    handler::Handler,
    http::Method,
    http::StatusCode,
    http::Uri,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::signal;

#[tokio::main]
async fn main() {
    println!("axum has run...");
    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user))
        .route("/html", get(html_data));

    let app = app.fallback(handler_no_router.into_service());

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown())
        .await
        .unwrap();
}

// basic handler that responds with a static string
// 返回一个静态的字符串字面量
async fn root() -> &'static str {
    "hello,rust"
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    id: i64,
    name: String,
}

// the input to our `create_user` handler
#[derive(Deserialize, Serialize, Debug)]
struct CreateUserReq {
    name: String,
}

// body request 自动反序列化为payload，它的类型是User
async fn create_user(Json(payload): Json<CreateUserReq>) -> impl IntoResponse {
    let u = User {
        id: 1,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(u))
}

// 返回一段html文本
async fn html_data() -> Html<&'static str> {
    Html("<h1>hello world</h1>")
}

// handle global 404 router not found
// 自动实现量handler::Handler trait的 into_service 方法
// fn into_service(self) -> IntoService<Self, T, B> {
//     IntoService::new(self)
// }
async fn handler_no_router(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("request method:{} request uri:{} not found", method, uri),
    )
}

// graceful shutdown
async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { // 按住ctrl +c 退出
            println!("server will shutdown")
        },
        _= terminate => {
            println!("received terminate signal")
        },
    }

    println!("signal received,starting graceful shutdown")
}
