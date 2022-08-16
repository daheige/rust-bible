use axum::extract::Form;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
pub struct User {
    id: i64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Reply<T> {
    code: i64,
    message: String,
    data: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmptyObject {}

type EmptyArray = Vec<EmptyObject>;

// 接收json格式，自动序列化为CreateUserReq格式
pub async fn create_user(Json(payload): Json<CreateUserReq>) -> impl IntoResponse {
    let user = User {
        id: 1,
        name: payload.username,
    };

    (StatusCode::OK, Json(user))
}

// Returning different response types
// http://localhost:3000/either?id=2&name=daheige
pub async fn either(Form(user): Form<User>) -> Response {
    println!("current user:{} id:{}", user.name, user.id);
    if user.id > 100 {
        let res = Reply {
            code: 1000,
            message: "data not found".to_string(),
            data: EmptyObject {},
        };

        let body = serde_json::to_string(&res).unwrap();
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response()
    } else {
        if user.id == 1 {
            let empty_arr: EmptyArray = Vec::new();
            let res = Reply {
                code: 1001,
                message: "data not found".to_string(),
                data: empty_arr,
            };

            let body = serde_json::to_string(&res).unwrap();
            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                body,
            )
                .into_response();
        }

        (
            StatusCode::OK,
            Json(Reply {
                code: 0,
                message: "ok".to_string(),
                data: user,
            }),
        )
            .into_response()
    }
}
