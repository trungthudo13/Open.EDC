// src/identity/handlers/login_handler.rs

use crate::identity::schemas::login::{LoginRequest, LoginResponse};
use axum::{Json, http::StatusCode, response::IntoResponse};

#[utoipa::path(
    post,
    tag = "Identity",
    tags = ["Identity"],
    operation_id  = "Login",
    path = "/api/identity/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "OK", body = LoginResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn handle(
    login_request: Json<LoginRequest>,
    // ) -> Result<impl IntoResponse, StatusCode> {
) -> impl IntoResponse {
    let Json(instance) = login_request;
    println!("Received login request: {:?}", instance);
    (StatusCode::OK, Json(LoginResponse::new2(instance)))
}
