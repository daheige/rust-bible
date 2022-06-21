use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn home() -> &'static str {
    "hello,axum"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserReq {
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i64,
    name: String,
}

// 接收json格式，自动序列化为CreateUserReq格式
pub async fn create_user(Json(payload): Json<CreateUserReq>) -> impl IntoResponse {
    let user = User {
        id: 1,
        name: payload.username,
    };

    (StatusCode::OK, Json(user))
}
