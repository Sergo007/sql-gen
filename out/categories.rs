#[derive(sqlx::FromRow)]
pub struct Categories {
 pub id: i64,
 pub created_at: Option<chrono::DateTime<chrono::Utc>>,
 pub label: String,
}