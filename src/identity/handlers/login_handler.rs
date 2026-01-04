// src/identity/handlers/login_handler.rs

use crate::identity::dtos::login_request_dto::LoginRequestDto;
use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn handle(
    Json(login_request): Json<LoginRequestDto>,
    // ) -> Result<impl IntoResponse, StatusCode> {
) -> impl IntoResponse {
    println!("Received login request: {:?}", login_request);
    (StatusCode::OK, Json(login_request))
}
