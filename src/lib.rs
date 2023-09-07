use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

mod database;
mod routes;

#[derive(Clone)]
struct ServerCtx {
    db: database::Database,
}

pub async fn run(db_url: &str) {
    tracing_subscriber::fmt::init();

    let db = database::Database::new();
    let ctx = ServerCtx { db };

    let app = Router::new()
        .route("/", get(routes::root))
        .with_state(ctx)
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
