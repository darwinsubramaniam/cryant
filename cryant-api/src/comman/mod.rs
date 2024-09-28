use axum::{http::StatusCode, response::IntoResponse};

pub mod app_error;
pub mod app_route;


pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "This route does not exist.")
}