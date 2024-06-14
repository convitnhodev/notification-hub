use crate::adapters::spi::db::models::Notification;
use crate::domain::notification_entity::NotificationEntity;
use crate::application::mappers::db_mapper::DbMapper;


pub struct NotificationDbMapper{}


impl DbMapper<NotificationEntity, Notification> for NotificationDbMapper {
    fn to_db(entity: NotificationEntity) -> Notification {
        Notification {
           
        }
    }

    fn to_entity(model: Notification) -> NotificationEntity {
        NotificationEntity {
           
        }
}