use axum::response::IntoResponse;

pub async fn root() -> impl IntoResponse {
    "Hewwo wowd"
}
