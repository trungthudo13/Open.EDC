// src/common/schemas/openapi.rs

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::identity::handlers::login_handler::handle,
    crate::identity::handlers::userinfo_handler::handle
))]
pub struct OpenApiDoc;
