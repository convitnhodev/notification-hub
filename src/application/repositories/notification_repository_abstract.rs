use async_trait::async_trait;



use crate::domain::notification_entity::NotificationEntity; 



#[async_trait(?Send)]
pub trait NotificationRepositoryAbstract {
    async fn create(&self) -> Result<CatFactEntity, Box<dyn Error>>;
    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>>;
}