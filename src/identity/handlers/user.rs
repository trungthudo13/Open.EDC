// src/identity/handlers/login_handler.rs

#[utoipa::path(
    get,
    tag = "Identity",
    tags = ["Identity"],
    operation_id  = "User Info",
    path = "/api/identity/v1/userinfo",
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn handler() -> &'static str {
    "Hello, World!"
}
