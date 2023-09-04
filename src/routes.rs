use axum::response::IntoResponse;

pub(crate) async fn root() -> impl IntoResponse {
    "Hewwo wowd"
}
