// src/main.rs
mod handlers;
mod routers;

use crate::routers::identities::router as identities_router;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a single route
    // our router
    async fn root() -> &'static str {
        "Hello, Worldsaasa!"
    }

    let app = Router::new()
        .route("/", get(root))
        .merge(identities_router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
