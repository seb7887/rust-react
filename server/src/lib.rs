#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::http::Method;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

#[macro_use]
extern crate diesel;

use rocket_contrib::json::JsonValue;

mod db;
mod models;
mod schema;

fn cors_options() -> Cors {
  let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8000"]);

  CorsOptions {
    allowed_origins,
    allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
    allowed_headers: AllowedHeaders::some(&[
      "Authorization",
      "Accept",
      "Access-Control-Allow-Origin",
    ]),
    allow_credentials: true,
    ..Default::default()
  }
  .to_cors()
  .expect("Error while building CORS")
}

#[get("/world")]
fn world() -> &'static str {
  "Hello world!"
}

#[catch(404)]
fn not_found() -> JsonValue {
  json!({
    "status": "error",
    "reason": "Not found"
  })
}

pub fn rocket() -> rocket::Rocket {
  rocket::ignite()
    .attach(db::DbConn::fairing())
    .attach(cors_options())
    .mount("/hello", routes![world])
    .register(catchers![not_found])
}
