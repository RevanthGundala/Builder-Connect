use crate::schema::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    
    // Optional fields -> Field to Weighting (How much it matters to the user)
    pub github: Option<String>, // if they are non-technical...
    pub website: Option<String>, // if they are non-technical...
    pub age: Option<u32>, 
    pub age_weight: Option<i32>,
    pub location: Option<String>,
    pub location_weight: Option<i32>,
    pub employer: Option<String>, // school / work / etc
    pub employer_weight: Option<i32>,
    pub reason: Option<String>, // why they want to join (personal project/startup)
    pub interests: Option<String>, // what they're interested in (crpyo, ML, etc)
    pub interests_weight: Option<i32>,
    pub skills: Option<String>, // what tech stack they want to work on (web dev, ML, etc)
    pub skills_weight: Option<i32>
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,

    pub github: Option<&'a str>,
    pub website: Option<&'a str>,
    pub age: Option<u32>,
    pub age_weight: Option<i32>,
    pub location: Option<&'a str>,
    pub location_weight: Option<i32>,
    pub employer: Option<&'a str>,
    pub employer_weight: Option<i32>,
    pub reason: Option<&'a str>,
    pub interests: Option<&'a str>,
    pub interests_weight: Option<i32>,
    pub skills: Option<&'a str>,
    pub skills_weight: Option<i32>
}