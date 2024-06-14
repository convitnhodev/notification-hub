use async_trait::async_trait;



use crate::domain::notification_entity::NotificationEntity; 



#[async_trait(?Send)]
pub trait NotificationRepositoryAbstract {
    async fn create_new_notification(&self, notification_entity: NotificationEntity) -> Result<String, Box<Error>> ;
    async fn get_by_id(&self, id: String) -> Result<NotificationEntity, Box<Error>>;
}