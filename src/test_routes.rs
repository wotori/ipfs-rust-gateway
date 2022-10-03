use std::io::Cursor;
use rocket::tokio::time::{sleep, Duration};

use rocket_okapi::{openapi};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};

#[openapi(tag = "ipfs")]
#[post("/")]
pub async fn upload_file() -> String {
    let client = IpfsClient::default();
    let file = Cursor::new("Hello, world!");
    match client.add(file).await {
        Ok(res) => res.hash,
        Err(e) => String::from("error")
    }
}