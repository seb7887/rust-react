use crate::schema::books;

#[derive(Queryable)]
pub struct Book {
  id: u32,
  user_id: u32,
  title: String,
  author: String,
  cover: Option<String>,
  page_count: u32,
  publisher: Option<String>,
  synopsis: Option<String>,
  created_at: u64,
  updated_at: u64,
}
