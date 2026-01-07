// src/identity/handlers/authorize_handler.rs

use crate::identity::schemas::login::{LoginRequestDto, LoginResponseDto};
use axum::{Json, http::StatusCode, response::IntoResponse};

#[utoipa::path(
    post,
    tag = "Identity",
    tags = ["Identity"],
    operation_id  = "Login",
    path = "/api/auth/login",
    request_body =  LoginRequestDto,
    responses(
        (status = 200, description = "OK", body = LoginResponseDto),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn handle(
    Json(req_body): Json<LoginRequestDto>,
    // ) -> Result<impl IntoResponse, StatusCode> {
) -> impl IntoResponse {
    println!("Received login request: {:?}", req_body);
    (StatusCode::OK, Json(LoginResponseDto::new(req_body)))
}
