#[macro_use]
extern crate rocket;

mod test_routes;

use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*};

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                test_routes::index,
                test_routes::retrieve,
                test_routes::upload
            ],
        )
        .mount(
            "/doc",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        ).launch().await;
}

