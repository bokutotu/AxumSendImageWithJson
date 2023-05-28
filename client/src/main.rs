use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() {
    let image = image::open("../image/penguin.jpg").unwrap();
    let bytes = image.as_bytes().to_vec();
    let image_part = reqwest::multipart::Part::bytes(bytes);
    let form = reqwest::multipart::Form::new()
        .part("image", image_part)
        .part("user_without_bytes", reqwest::multipart::Part::text(serde_json::to_string(&User {
            name: "penguin".to_string(),
            age: 10,
            bytes: vec![],
        }).unwrap()));
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3000/image")
        .multipart(form)
        .send()
        .await
        .unwrap();

    println!("{:?}", response.status());
    let response_inner: Response = response.json().await.unwrap();
    println!("{:?}", response_inner.bytes);
}
