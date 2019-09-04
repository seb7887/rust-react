use crate::db::{self, user::CreateUserError};
use crate::errors::{Errors, FieldValidator};

use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewUser {
  user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
  #[validate(length(min = 1))]
  username: Option<String>,
  #[validate(email)]
  email: Option<String>,
  #[validate(length(min = 8))]
  password: Option<String>,
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(new_user: Json<NewUser>, conn: db::DbConn) -> Result<JsonValue, Errors> {
  let new_user = new_user.into_inner().user;

  let mut extractor = FieldValidator::validate(&new_user);
  let username = extractor.extract("username", new_user.username);
  let email = extractor.extract("email", new_user.email);
  let password = extractor.extract("password", new_user.password);

  extractor.check()?;

  db::user::create(&conn, &username, &email, &password)
    .map(|user| json!({ "user": user.to_user_auth() }))
    .map_err(|error| {
      let field = match error {
        CreateUserError::DuplicatedEmail => "email",
        CreateUserError::DuplicatedUsername => "username",
      };
      Errors::new(&[(field, "has already been taken")])
    })
}
