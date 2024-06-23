use crate::adapters::spi::db::models::Notification;
use crate::domain::notification_entity::NotificationEntity;
use crate::application::mappers::db_mapper::DbMapper;
use chrono::NaiveDateTime;


pub struct NotificationDbMapper{}


impl DbMapper<NotificationEntity, Notification> for NotificationDbMapper {
    fn to_db(entity: NotificationEntity) -> Notification {
        let now = chrono::Utc::now().naive_utc();
        Notification {
            id: entity.id, 
            from_user: entity.from_user, 
            to_user: entity.to_user, 
            content: Some(entity.content), 
            status: entity.status.to_string(), 
            created_at: now,
            updated_at: now,
            viewed_at: None, 
        }
    }

    fn to_entity(model: Notification) -> NotificationEntity {
        NotificationEntity {
            id: model.id,
            from: model.from_user,
            to: model.to_user,
            content: model.content.unwrap_or_default(), 
            status: model.status.parse().unwrap_or(Status::Pending), 
        }
    }
}