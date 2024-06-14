use std::time::{SystemTime};


#[derive(Debug, Clone)]



pub struct SessionEntity {
    pub id: String, 
    pub token: String, 
    pub expire_at: SystemTime,  
}


pub struct IdentityEntity {
    pub id: String,
    pub email: String,
    pub password: String,
    pub password_changed_at: SystemTime,
    pub session: SessionEntity,
}


