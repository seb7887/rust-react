#[derive(Queryable)]
pub struct Book {
  pub id: u32,
  pub user_id: u32,
  pub title: String,
  pub author: String,
  pub cover: Option<String>,
  pub page_count: u32,
  pub publisher: Option<String>,
  pub synopsis: Option<String>,
  pub created_at: u64,
  pub updated_at: u64,
}
