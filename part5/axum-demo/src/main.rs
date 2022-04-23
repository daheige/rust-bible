use async_trait::async_trait;
use axum::{
    extract::{Form, FromRequest, RequestParts},
    handler::Handler,
    http::Method,
    http::StatusCode,
    http::Uri,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    BoxError, Json, Router,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::net::SocketAddr;
use tokio::signal;
// 验证的validator
use thiserror::Error;
use validator::Validate;

#[tokio::main]
async fn main() {
    println!("axum has run...");
    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user))
        .route("/html", get(html_data))
        .route("/form", get(show_form).post(post_form))
        .route("/form2", get(form2));
    // .route("/form", get(show_form))
    // .route("/form", post(post_form));

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

// form
async fn show_form() -> Html<&'static str> {
    Html::from(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>
                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>
                    <input type="submit" value="Subscribe">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug, Validate)]
#[allow(dead_code)]
struct Input {
    #[validate(length(min = 1, message = "name can not be empty"))]
    name: String,
    #[validate(length(min = 1, message = "email can not be empty"))]
    email: String,
}

async fn post_form(Form(input): Form<Input>) -> impl IntoResponse {
    println!("{:?}", input);
    println!("email:{}", input.email);
    (
        StatusCode::OK,
        format!("name:{},emil:{}", input.name, input.email),
    )
}

//==========validator========
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

// 当请求http://localhost:3000/form2?name=111&email=
// 会提示：Input validation error: [email: email can not be empty]
async fn form2(ValidatedForm(input): ValidatedForm<Input>) -> Html<String> {
    println!("{:?}", input);
    println!("email:{}", input.email);
    Html(format!("name:{},emil:{}", input.name, input.email))
}

// FromRequest 需要实现 from_request 方法
#[async_trait]
impl<T, B> FromRequest<B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req).await?; // 从请求中结构value
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
}

// 这里不需要实现 Display 因为 thiserror::Error trait实现了
// impl fmt::Display for ServerError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

// 为ServerError实现IntoResponse
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                // 这里最好是给 ServerError::ValidationError 实现Display trait
                let message = format!("input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
