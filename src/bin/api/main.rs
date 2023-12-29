use axum::middleware::from_extractor;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Router};
use tokio::net::TcpListener;

use clio_db::auth::{get_public_jwk, AuthorizationMiddleware, Authorized, Jwks};
use clio_db::db_api::endpoints;

#[tokio::main]
async fn main() {
    let jwks = Jwks(vec![get_public_jwk()]);
    let router = Router::new()
        .route("/db", get(endpoints::get))
        .route_layer(from_extractor::<AuthorizationMiddleware>())
        .route("/ping", get(ping))
        .route_layer(from_extractor::<AuthorizationMiddleware>())
        .route("/login", post(endpoints::login))
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
