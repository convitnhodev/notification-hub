use crate::adapters::spi::{db::db_notification_repository::NotificationRepository};

pub struct AppState {
    pub app_name: String,
    pub notification_repository : NotificationRepository,
}