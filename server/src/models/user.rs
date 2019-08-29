#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub token: Option<String>,
  pub created_at: u64,
  pub updated_at: u64,
}
