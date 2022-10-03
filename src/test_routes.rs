use std::fs::File;
use std::io::Cursor;

use rocket_okapi::{openapi};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use rocket::{Data, Response};

#[openapi(tag = "ipfs")]
#[post("/", data = "<paste>")]
pub async fn upload_file(
    paste: Data<'_>,
) -> String {
    let client = IpfsClient::default();
    let file = Cursor::new("Hello, world!");
    println!("saving file...");
    match client.add(file).await { // TODO: find a way how to put file here
        Ok(res) => res.hash.clone(),
        Err(_e) => String::from("error")
    }
}