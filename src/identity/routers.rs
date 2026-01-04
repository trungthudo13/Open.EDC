// src/identity/routers.rs
use super::handlers::{
    login_handler::handle as login_handler, userinfo_handler::handle as userinfo_handler,
};
use axum::{Router, routing::{get, post}};

pub fn router() -> Router {
    return Router::new()
        .route("/api/identity/v1/login", get(login_handler))
        .route("/api/identity/v1/authorize", post(userinfo_handler))
        .route("/api/identity/v1/revoke", post(userinfo_handler))
        .route("/api/identity/v1/userinfo", post(userinfo_handler));
    // .route("/foo/bar", get(root));
}
