use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use mongodb::bson::doc;
use chrono::{Local, DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Time {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
    pub id: Viewability,
    pub first_name: Viewability,
    pub last_name: Viewability,
    pub email: Viewability,
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

impl VectorEmbedding {
    pub fn default() -> Self {
        VectorEmbedding {
            age: vec![],
            location: vec![],
            employer: vec![],
            reason: vec![],
            project_interests: vec![],
            personality_interests: vec![],
            skills: vec![],
            right_swipes: vec![],
            left_swipes: vec![],
            matches: vec![],
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub github: Option<String>, 
    pub website: Option<String>, 
    pub age: Option<i32>, 
    pub location: Option<String>,
    pub employer: Option<String>, // school / work / etc
    pub reason: Option<String>, // why they want to join (personal project/startup)
    pub project_interests: Option<String>, // what they're interested in (crpyo, ML, etc)
    pub personality_interests: Option<String>,
    pub skills: Option<String>, // what tech stack they want to work on (web dev, ML, etc)
    pub right_swipes: Option<Vec<String>>, // list of user's ids who this user has swiped right on
    pub left_swipes: Option<Vec<String>>, // list of user's ids who this user has swiped left on
    pub matches: Option<Vec<String>>, // list of user
    pub public_fields: Option<UserView>, // list of fields that are public
    pub vector_embeddings: Option<VectorEmbedding>,
}