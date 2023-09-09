use common_data::*;
use std::sync::Arc;

use axum::{routing::get, Router};
use rstar::RTree;
use tower_http::trace::TraceLayer;

mod database;
mod routes;

#[derive(Clone)]
pub struct ServerCtx {
    db: database::Database,
    tree: RTree<RoadDamage>,
}

impl ServerCtx {
    pub async fn new() -> Self {
        let db = database::Database::new().await;
        let tree = RTree::bulk_load(db.get_all_records().await);
        Self { db, tree }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let ctx = ServerCtx::new().await;

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/points", get(routes::get_points_in_rect))
        .with_state(Arc::new(ctx))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}