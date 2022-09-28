#[macro_use]
extern crate rocket;

use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/sleep/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index])
        .mount("/test", routes![delay]).launch().await;
}

