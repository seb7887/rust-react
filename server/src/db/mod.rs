use rocket_contrib::databases::diesel;

pub mod user;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);
