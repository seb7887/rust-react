use rocket_contrib::databases::diesel;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);
