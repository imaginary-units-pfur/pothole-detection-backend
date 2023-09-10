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
        db.create_mock_data()
            .await
            .expect("Failed to create mock data"); // This is behind a feature flag, so it does not generate any data if that is turned off.
        let tree = RTree::bulk_load(
            db.get_all_records()
                .await
                .expect("Failed to load all points for bulk-loading"),
        );
        Self { db, tree }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let ctx = ServerCtx::new().await;

    let app = Router::new()
        .route("/", get(routes::root))
        .route(
            "/points/by-coords/from/:x1/:y1/to/:x2/:y2",
            get(routes::get_points_in_rect),
        )
        .route(
            "/points/by-id/:id",
            get(routes::get_additional_info_for_point),
        )
        .with_state(Arc::new(ctx))
        .layer(TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::very_permissive());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
