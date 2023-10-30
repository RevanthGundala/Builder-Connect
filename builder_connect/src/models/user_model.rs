use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use mongodb::bson::doc;
use chrono::{DateTime, Utc};

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

impl UserView{
    fn default() -> Self {
        UserView {
            first_name: Viewability::Private,
            last_name: Viewability::Private,
            email: Viewability::Private,
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

// impl VectorEmbedding {
//     pub fn default() -> Self {
//         VectorEmbedding {
//             age: vec![],
//             location: vec![],
//             employer: vec![],
//             reason: vec![],
//             project_interests: vec![],
//             personality_interests: vec![],
//             skills: vec![],
//             right_swipes: vec![],
//             left_swipes: vec![],
//             matches: vec![],
//         }
//     }

//     pub fn to_vec(&self) -> Vec<f32> {
//         vec![
//             self.age.clone(),
//             self.location.clone(),
//             self.employer.clone(),
//             self.reason.clone(),
//             self.project_interests.clone(),
//             // self.personality_interests.clone(),
//             // self.skills.clone(),
//             // self.right_swipes.clone(),
//             // self.left_swipes.clone(),
//             // self.matches.clone(),
//         ].into_iter().flatten().collect()
//     }
// }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sub_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub github: Option<String>, 
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
    pub matches: Option<Vec<String>>, // list of user
    pub public_fields: Option<UserView>, // list of fields that are public
    pub vector_embeddings: Option<Vec<f32>>,
}

impl User {
    pub fn new(sub_id: String) -> Self {
        User {
            id: Some(ObjectId::new()),
            sub_id: Some(sub_id),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            github: Some("".to_string()),
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
            public_fields: Some(UserView::default()),
            vector_embeddings: Some(vec![]),
        }
    } 
}