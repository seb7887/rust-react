use crate::models::user::User;
use crate::schema::users;
use crypto::scrypt::{scrypt_simple, ScryptParams};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct CreateUser<'a> {
  pub email: &'a str,
  pub username: &'a str,
  pub token: &'a str,
}

pub enum CreateUserError {
  DuplicatedEmail,
  DuplicatedUsername,
}

impl From<Error> for CreateUserError {
  fn from(err: Error) -> CreateUserError {
    if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
      match info.constraint_name() {
        Some("users_username_key") => return CreateUserError::DuplicatedUsername,
        Some("users_email_key") => return CreateUserError::DuplicatedEmail,
        _ => {}
      }
    }
    panic!("Error creating user: {:?}", err)
  }
}

pub fn create(
  conn: &PgConnection,
  username: &str,
  email: &str,
  password: &str,
) -> Result<User, CreateUserError> {
  let token = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");

  let new_user = &CreateUser {
    username: username,
    email: email,
    token: token,
  };

  diesel::insert_into(users::table)
    .values(new_user)
    .get_result::<User>(conn)
    .map_err(Into::into)
}
