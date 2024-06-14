#[derive(Debug)]
pub enum Status {
    StatusReady, 
    StatusProcessing,
    StatusFailed,
    StatusSuccess, 
    StatusPending,
    StatusViewed,
}


#[derive(Debug)]
pub struct NotificationEntity {
    pub id: String,
    pub from: String,
    pub to: String,
    pub content: String,
    pub status: Status,
}



impl NotificationEntity {
    pub fn new(id: String, from: String, to: String, content: String) -> Self {
        NotificationEntity {
            id: id, 
            from: from,
            content: content,
            to: to,
            status: Status::StatusReady
        }
    }
}