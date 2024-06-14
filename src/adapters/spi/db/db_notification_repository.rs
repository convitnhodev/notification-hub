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
}
