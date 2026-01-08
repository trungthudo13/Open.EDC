// src/identity/handlers/authorize_handler.rs

use crate::auth::schemas::login::{LoginRequestDto, LoginResponseDto};
use axum::{Json, http::StatusCode, response::IntoResponse};

#[utoipa::path(
    post,
    tag = "Identity",
    tags = ["Identity"],
    operation_id = "Login",
    path = "/api/auth/login",
    responses(
        (status = StatusCode::OK, description = "OK"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized")
    )
)]
pub async fn handler(
    Json(req_body): Json<LoginRequestDto>,
    // ) -> Result<impl IntoResponse, StatusCode> {
) -> impl IntoResponse {
    println!("Received login request: {:?}", req_body);
    (StatusCode::OK, Json(LoginResponseDto::new(req_body)))
}
