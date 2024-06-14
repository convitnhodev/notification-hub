use crate::adapters::spi::db::schema::*;

#[derive(Queryable, QueryableByName, Selectable)]
#[table_name = "notifications"]
#[diesel(check_for_backend(diesel::pg::Pg))]
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