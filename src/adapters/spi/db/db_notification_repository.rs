use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;


use crate::adapters::spi::db::{db_connection::DbConnection};
use crate::application::{repositories::NotificationRepositoryAbstract};
use crate::domain::notification_entity::NotificationEntity; 



pub struct NotificationRepository {
    pub db_connection: DbConnection,
}


#[async_trait(?Send)]
impl NotificationRepositoryAbstract for NotificationRepository {
    async fn create_new_notification(&self, notification_entity NotificationEntity) -> Result<NotificationEntity, Box<Error>> { 
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        diesel::insert_into
    }


    async fn update_viewed_time_and_status(&self, id: String, user_id: String, viewed_time :SystemTime) -> Result<String, Box<Error>> { 

    }


    async fn update_viewed_time_and_status(&self, id: String, user_id: String, viewed_time :SystemTime) -> Result<String, Box<Error>> { 
        
    }


    async fn update_viewed_time_and_status_for_entire_notification(&self, user_id: String, viewed_time :SystemTime) -> Result<String, Box<Error>> { 
        
    }


    async fn update_status_by_id(&self, user_id: String, status Status) -> Result<String, Box<Error>> { 
        
    }


    async fn get_by_id(&self, id String) Result<NotificationEntity, Box<Error>> {
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let results = dog_facts.load::<Notification>(&conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(DogFactDbMapper::to_entity).collect::<Vec<DogFactEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }





}
