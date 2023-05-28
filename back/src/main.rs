use axum::routing::{get, post};
use axum::extract::Multipart;
use serde::{Deserialize, Serialize};

use tower_http::limit::RequestBodyLimitLayer;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    bytes: Vec<u8>,

}

#[derive(Serialize, Deserialize)]
struct UserWithoutBytes {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct Response {
    bytes: Vec<u8>,
}

/// User構造体を受け取り、bytesをJSONにして返す(Response)
async fn user_handler(mut multipart: Multipart) -> axum::response::Json<Response> {
    let image = multipart.next_field().await.unwrap().unwrap();
    let bytes = image.bytes().await.unwrap();
    let user_without_bytes = multipart.next_field().await.unwrap().unwrap();
    let user_without_bytes: UserWithoutBytes = serde_json::from_slice(&user_without_bytes.bytes().await.unwrap()).unwrap();
    let user = User {
        name: user_without_bytes.name,
        age: user_without_bytes.age,
        bytes: bytes.to_vec(),
    };
    axum::response::Json(Response {
        bytes: user.bytes,
    })
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/image", post(user_handler))
        .layer(RequestBodyLimitLayer::new(
            2014 * 1024 * 1024, /* 1GB */
        ));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
