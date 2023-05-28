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
    let image_part_1 = reqwest::multipart::Part::bytes(bytes.clone());
    let image_part_2 = reqwest::multipart::Part::bytes(bytes.clone());
    let image_part_3 = reqwest::multipart::Part::bytes(bytes.clone());
    let image_part_4 = reqwest::multipart::Part::bytes(bytes.clone());
    let form = reqwest::multipart::Form::new()
        .part("image", image_part_1)
        .part("image", image_part_2)
        .part("image", image_part_3)
        .part("image", image_part_4)
        .part(
            "user",
            reqwest::multipart::Part::text(
                serde_json::to_string(&User {
                    name: "penguin".to_string(),
                    age: 10,
                    bytes: vec![],
                })
                .unwrap(),
            ),
        );
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/image")
        .multipart(form)
        .send()
        .await
        .unwrap();

    println!("{:?}", response.status());
    let response_inner: Response = response.json().await.unwrap();
    println!("{:?}", response_inner.bytes);
}
