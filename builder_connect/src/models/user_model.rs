use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    // Optional fields -> Field to Weighting (How much it matters to the user)
    pub github: Option<String>, // if they are non-technical...
    pub website: Option<String>, // if they are non-technical...
    pub age: Option<i32>, 
    pub age_weight: Option<i32>,
    pub location: Option<String>,
    pub location_weight: Option<i32>,
    pub employer: Option<String>, // school / work / etc
    pub employer_weight: Option<i32>,
    pub reason: Option<String>, // why they want to join (personal project/startup)
    pub project_interests: Option<String>, // what they're interested in (crpyo, ML, etc)
    pub project_interests_weight: Option<i32>,
    pub personality_interests: Option<String>,
    pub personality_interests_weight: Option<i32>,
    pub skills: Option<String>, // what tech stack they want to work on (web dev, ML, etc)
    pub skills_weight: Option<i32>,
    pub right_swipes: Option<Vec<i32>>, // list of user's ids who this user has swiped right on
    pub left_swipes: Option<Vec<i32>>, // list of user's ids who this user has swiped left on
    pub incoming_right_swipes: Option<Vec<i32>>, 
    pub incoming_left_swipes: Option<Vec<i32>>, // list of user's ids who have swiped right on this user
    pub matches: Option<Vec<i32>>, // list of user
}

