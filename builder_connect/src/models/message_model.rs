use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub room_id: String,
    pub user_sub_id: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl Message{
    pub fn new(
        room_id: String, 
        user_sub_id: String, 
        content: String,
        created_at: Option<DateTime<Utc>>) -> Self {
        Message{
            id: Some(ObjectId::new()),
            room_id,
            user_sub_id,
            content,
            created_at
        }
    }
}