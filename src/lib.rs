use axum::{routing::get, Router};
mod routes;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct ServerCtx {
    db_client: redis::Client,
}

pub async fn run(db_url: &str) {
    tracing_subscriber::fmt::init();

    let client = redis::Client::open(db_url).unwrap();
    let ctx = ServerCtx { db_client: client };
    let app = Router::new()
        .route("/", get(routes::root))
        .with_state(ctx)
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
