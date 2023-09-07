use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

mod database;
mod models;
mod routes;

#[derive(Clone)]
pub struct ServerCtx {
    db: database::Database,
}

impl ServerCtx {
    pub async fn new() -> Self {
        let db = database::Database::new().await;
        Self { db }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let ctx = ServerCtx::new().await;

    let app = Router::new()
        .route("/", get(routes::root))
        .with_state(ctx)
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
