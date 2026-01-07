mod consts;
mod identity;

// use crate::common::schemas::openapi::OpenApiDoc;
use crate::identity::routers::router as identities_router;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::identity::handlers::authorize_handler::handle,
    crate::identity::handlers::userinfo_handler::handle
))]
pub struct OpenApiDoc;

#[tokio::main]
async fn main() {
    // let openapi = OpenApiDoc::openapi();
    let app = Router::new()
        .merge(identities_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", OpenApiDoc::openapi()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
