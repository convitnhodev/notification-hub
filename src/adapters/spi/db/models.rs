use crate::adapters::spi::db::schema::*;

#[derive(Queryable, QueryableByName)]
#[table_name = "notifications"]
pub struct Notification {
    pub id: String,
    pub from_user: String,
    pub to_user: String,
    pub content: Option<String>, 
    pub status: String,
    pub created_at: chrono::NaiveDateTime, 
    pub updated_at: chrono::NaiveDateTime,
    pub viewed_at: Option<chrono::NaiveDateTime>,
}