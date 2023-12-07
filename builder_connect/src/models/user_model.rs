use mongodb::bson::{oid::ObjectId, Uuid};
use serde::{Serialize, Deserialize};
use mongodb::bson::doc;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Access {
    Admin,
    User,
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Viewability {
    Public,
    Private,
} 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserView{
    pub username: Viewability,
    pub email: Viewability,
    pub discord: Viewability,
    pub github: Viewability,
    pub website: Viewability,
    pub age: Viewability,
    pub location: Viewability,
    pub employer: Viewability,
    pub reason: Viewability,
    pub project_interests: Viewability,
    pub personality_interests: Viewability,
    pub skills: Viewability,
}

impl UserView{
    pub fn default() -> Self {
        UserView {
            username: Viewability::Public,
            email: Viewability::Private,
            discord: Viewability::Private,
            github: Viewability::Private,
            website: Viewability::Private,
            age: Viewability::Private,
            location: Viewability::Private,
            employer: Viewability::Private,
            reason: Viewability::Private,
            project_interests: Viewability::Private,
            personality_interests: Viewability::Private,
            skills: Viewability::Private,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorEmbedding{
    pub age: Vec<f32>,
    pub location: Vec<f32>,
    pub employer: Vec<f32>,
    pub reason: Vec<f32>,
    pub project_interests: Vec<f32>,
    pub personality_interests: Vec<f32>,
    pub skills: Vec<f32>,
    pub right_swipes: Vec<f32>,
    pub left_swipes: Vec<f32>,
    pub matches: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    pub room_id: Uuid,
    pub user_sub_ids: Vec<String>, // will generally be 1 user (match)
}

impl Room {
    pub fn new(room_id: Uuid, user_match_sub_id: String) -> Self {
        Room {
            room_id,
            user_sub_ids: vec![user_match_sub_id],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sub_id: Option<String>,
    pub image_url: String,
    pub username: String,
    pub email: String,
    pub discord: String,
    pub website: Option<String>, 
    pub age: Option<String>, 
    pub location: Option<String>,
    pub employer: Option<String>, // school / work / etc
    pub reason: Option<String>, // why they want to join (personal project/startup)
    pub project_interests: Option<String>, // what they're interested in (crpyo, ML, etc)
    pub personality_interests: Option<String>,
    pub skills: Option<String>, // what tech stack they want to work on (web dev, ML, etc)
    pub right_swipes: Option<Vec<String>>, // list of user's ids who this user has swiped right on
    pub left_swipes: Option<Vec<String>>, // list of user's ids who this user has swiped left on
    pub matches: Option<Vec<Room>>, // list of user
    pub cannot_match: Option<Vec<String>>, // list of user's ids who this user cannot match with
    pub public_fields: Option<UserView>, // list of fields that are public
    pub vector_embeddings: Option<Vec<f32>>,
    pub last_seen: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(sub_id: String) -> Self {
        User {
            id: Some(ObjectId::new()),
            sub_id: Some(sub_id.clone()),
            image_url: "".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            discord: "".to_string(),
            website: Some("".to_string()),
            age: Some("".to_string()),
            location: Some("".to_string()),
            employer: Some("".to_string()),
            reason: Some("".to_string()),
            project_interests: Some("".to_string()),
            personality_interests: Some("".to_string()),
            skills: Some("".to_string()),
            right_swipes: Some(vec![]),
            left_swipes: Some(vec![]),
            matches: Some(vec![]),
            cannot_match: Some(vec![sub_id]),
            public_fields: Some(UserView::default()),
            vector_embeddings: Some(vec![]),
            last_seen: Some(Utc::now()),
        }
    } 
}