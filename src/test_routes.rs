use rocket_okapi::{openapi};
use std::path::Path;
use rocket::tokio::fs::File;

mod paste_id;

use paste_id::PasteId;
use rocket::data::{Data, ToByteUnit};

use ipfs_api_backend_hyper::{Error, IpfsApi, IpfsClient};
use std::io::Cursor;
use ipfs_api_backend_hyper::response::AddResponse;
use rocket::http::{Header, ContentType};
use rocket::serde::json::Json;
use std::fs::File as StdFile;

#[openapi(tag = "ipfs")]
#[get("/")]
pub fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[openapi(tag = "ipfs")]
#[get("/<id>")]
pub async fn retrieve(id: &str) -> Option<File> {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "files");
    let filename = Path::new(upload_dir).join(id);
    File::open(&filename).await.ok()
}

// Notice how much nicer this implementation is! And this time, it's secure.
// #[openapi(tag = "ipfs")]
// #[get("/<id>")]
// async fn retrieve(id: PasteId<'_>) -> Option<File> {
//     File::open(id.file_path()).await.ok()
// }

// In a real application, these would be retrieved dynamically from a config.
const ID_LENGTH: usize = 3;

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
        Err(e) => String::from("unexpected error")
    };
    Ok((hash).to_string())
}