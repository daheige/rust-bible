mod api;

use axum::{
    routing::{get, post},
    Router,
};

pub fn create_app() -> Router {
    let app = Router::new()
        .route("/", get(api::home))
        .route("/users", post(api::create_user))
        .route("/either", get(api::either));
    app
}
