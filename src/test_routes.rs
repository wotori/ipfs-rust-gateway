use rocket::tokio::time::{sleep, Duration};
use rocket::{serde::json::Json};

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    user_id: u64,
    username: String,
    #[schemars(example = "example_email")]
    email: Option<String>,
}

fn example_email() -> &'static str {
    "test@example.com"
}

#[openapi(tag = "testing routes")]
#[get("/hi")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "testing routes")]
#[get("/sleep/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[openapi(tag = "testing routes")]
#[get("/user")]
pub fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}