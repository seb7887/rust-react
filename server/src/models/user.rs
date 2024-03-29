use crate::auth::Auth;
use crate::schema::users;
use chrono::{Duration, Utc};
use serde::Serialize;

#[derive(Queryable, Identifiable, Serialize, Debug)]
#[table_name = "users"]
#[primary_key(id)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub bio: Option<String>,
  pub image: Option<String>,
  #[serde(skip_serializing)]
  pub token: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
  username: &'a str,
  email: &'a str,
  bio: Option<&'a str>,
  image: Option<&'a str>,
  token: String,
}

impl User {
  pub fn to_user_auth(&self) -> UserAuth {
    let exp = Utc::now() + Duration::days(60);
    let token = Auth {
      id: self.id,
      username: self.username.clone(),
      exp: exp.timestamp(),
    }
    .token();

    UserAuth {
      username: &self.username,
      email: &self.email,
      bio: self.bio.as_ref().map(String::as_str),
      image: self.image.as_ref().map(String::as_str),
      token,
    }
  }
}
