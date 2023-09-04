use axum::{routing::get, Router};
mod routes;

pub async fn run(db_url: &str) {
    let _client = redis::Client::open(db_url).unwrap();
    let app = Router::new().route("/", get(routes::root));
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
