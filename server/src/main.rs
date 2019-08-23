#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[get("/world")]
fn world() -> &'static str {
  "Hello world!"
}

fn main() {
  rocket::ignite()
    .attach(DbConn::fairing())
    .mount("/hello", routes![world])
    .launch();
}
