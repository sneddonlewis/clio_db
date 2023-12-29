use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::from_extractor;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use clio_db::models::LoginRequest;
use tokio::net::TcpListener;

use clio_db::auth::{encode_token, get_public_jwk, AuthorizationMiddleware, Authorized, Jwks};

#[tokio::main]
async fn main() {
    let jwks = Jwks(vec![get_public_jwk()]);
    let router = Router::new()
        .route("/ping", get(ping))
        .route_layer(from_extractor::<AuthorizationMiddleware>())
        .route("/login", post(login))
        .layer(Extension(jwks));

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
    println!("api");
}

async fn ping(Extension(claims): Extension<Authorized>) -> impl IntoResponse {
    println!("{}", claims.0.username);
    "pong".into_response()
}

async fn login(Json(request): Json<LoginRequest>) -> impl IntoResponse {
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
