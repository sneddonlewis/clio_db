use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    // add code here
    "db get".to_string()
}
