mod paste_id;

use rocket_okapi::{openapi};
use paste_id::PasteId;
use rocket::data::{Data, ToByteUnit};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::fs::File as StdFile;

const ID_LENGTH: usize = 5;

#[openapi(tag = "ipfs")]
#[post("/", data = "<paste>")]
pub async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(3.mebibytes()).into_file(id.file_path()).await?;
    Ok(("HOST, retrieve(id)").to_string())
}

#[openapi(tag = "ipfs")]
#[post("/ipfs", data = "<paste>")]
pub async fn upload_ipfs(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    let path = id.file_path();
    paste.open(3.mebibytes()).into_file(path).await?;
    let client = IpfsClient::default();
    let file = StdFile::open(id.file_name()).expect("could not read source file");
    let hash = match client.add(file).await {
        Ok(res) => res.hash,
        Err(_e) => String::from("unexpected error")
    };
    Ok((hash).to_string())
}