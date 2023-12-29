use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::Json;

use crate::auth::encode_token;
use crate::models::LoginRequest;

pub async fn get() -> impl IntoResponse {
    // add code here
    "db get".to_string()
}

pub async fn login(Json(request): Json<LoginRequest>) -> impl IntoResponse {
    if request.username == "root".to_string() && request.password == "root".to_string() {
        let token = encode_token(request.username.clone());
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::try_from(token).unwrap(),
        );
        (headers,).into_response()
    } else {
        (StatusCode::UNAUTHORIZED).into_response()
    }
}
