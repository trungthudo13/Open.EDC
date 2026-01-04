// src/identity/handlers/login_handler.rs

use crate::identity::schemas::login::{LoginRequest, LoginResponse};
use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn handle(
    login_request: Json<LoginRequest>,
    // ) -> Result<impl IntoResponse, StatusCode> {
) -> impl IntoResponse {
    let Json(instance) = login_request;
    println!("Received login request: {:?}", instance);
    (StatusCode::OK, Json(instance))
}
