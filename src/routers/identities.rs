// src/routers/identities.rs
use crate::handlers::identities::{authorize_v1, revoke_v1, token_v1, user_info_v1};
use axum::{Router, routing::post};

pub fn router() -> Router {
    return Router::new()
        .route("/api/v1/token", post(token_v1))
        .route("/api/v1/authorize", post(authorize_v1))
        .route("/api/v1/revoke", post(revoke_v1))
        .route("/api/v1/userinfo", post(user_info_v1));
    // .route("/foo/bar", get(root));
}
