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


impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::StatusReady => "ready".to_string(),
            Status::StatusProcessing => "processing".to_string(),
            Status::StatusFailed => "failed".to_string(),
            Status::StatusSuccess => "success".to_string(),
            Status::StatusPending => "pending".to_string(),
            Status::StatusViewed => "viewed".to_string(),
        }
    }
}
