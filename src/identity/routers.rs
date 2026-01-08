// // src/identity/routers.rs
// use axum::Router;
// use utoipa_axum::{router::OpenApiRouter, routes};

// pub fn router() -> Router {
//     let (router, _api) = OpenApiRouter::new()
//         .routes(routes!(
//             super::handlers::login::handler,
//             // super::handlers::userinfo_handler::handle
//         ))
//         .split_for_parts();

//     return router;
// }
