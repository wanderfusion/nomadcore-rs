use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: u16,
    data: String,
}

pub async fn health_check_handler() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse{
        status: axum::http::StatusCode::OK.as_u16(),
        data: "preparing my travel docs".to_string(),
    })
}