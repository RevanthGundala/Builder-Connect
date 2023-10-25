use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson, Document, doc, bson};

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

impl From<Viewability> for Bson {
    fn from(viewability: Viewability) -> Self {
        match viewability {
            Viewability::Public => Bson::String("Public".to_string()),
            Viewability::Private => Bson::String("Private".to_string()),
        }
    }
}

impl UserView {
    fn default() -> Self {
        UserView{
            id: Viewability::Private,
            first_name: Viewability::Public,
            last_name: Viewability::Public,
            email: Viewability::Public,
            github: Viewability::Public,
            website: Viewability::Public,
            age: Viewability::Public,
            location: Viewability::Public,
            employer: Viewability::Public,
            reason: Viewability::Public,
            project_interests: Viewability::Public,
            personality_interests: Viewability::Public,
            skills: Viewability::Public,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub right_swipes: Option<Vec<i32>>, // list of user's ids who this user has swiped right on
    pub left_swipes: Option<Vec<i32>>, // list of user's ids who this user has swiped left on
    pub incoming_right_swipes: Option<Vec<i32>>, 
    pub incoming_left_swipes: Option<Vec<i32>>, // list of user's ids who have swiped right on this user
    pub matches: Option<Vec<i32>>, // list of user
    pub public_fields: UserView, // list of fields that are public
}

