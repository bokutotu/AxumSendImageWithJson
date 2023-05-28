use axum::extract::{DefaultBodyLimit, Multipart};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

use tower_http::limit::RequestBodyLimitLayer;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserWithoutBytes {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    bytes: Vec<Vec<u8>>,
}

/// User構造体を受け取り、bytesをJSONにして返す(Response)
async fn user_handler(mut multipart: Multipart) -> axum::response::Json<Response> {
    let mut image_list = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_type = field.name();
        match field_type {
            Some("image") => {
                let bytes = field.bytes().await.unwrap();
                image_list.push(bytes.to_vec());
            }
            Some("user") => {
                let user_without_bytes: UserWithoutBytes =
                    serde_json::from_str(&field.text().await.unwrap()).unwrap();
                println!("{:?}", user_without_bytes);
            }
            _ => {
                println!("unknown field type");
            }
        }
    }
    axum::response::Json(Response { bytes: image_list })
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/image", post(user_handler))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            5 * 1014 * 1024 * 1024, /* 1GB */
        ));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
