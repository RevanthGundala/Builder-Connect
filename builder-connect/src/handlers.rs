use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::prelude::*;
use crate::errors::ServiceError;
use actix_web::{web, Error, HttpResponse};
use argonautica::input;
use diesel::dsl::{delete, insert_into, update};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
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
    pub incoming_right_swipes: Option<Vec<i32>>, // list of user's ids who have swiped right on this user
    pub incoming_left_swipes: Option<Vec<i32>>, // list of user's ids who have swiped left on this user
    pub matches: Option<Vec<i32>>, // list of 
}

impl From<User> for InputUser {
    fn from(user: User) -> InputUser {
        InputUser{
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            github: user.github.clone(),
            website: user.website.clone(),
            age: user.age,
            age_weight: user.age_weight,
            location: user.location.clone(),
            location_weight: user.location_weight,
            employer: user.employer.clone(),
            employer_weight: user.employer_weight,
            reason: user.reason.clone(),
            project_interests: user.project_interests.clone(),
            project_interests_weight: user.project_interests_weight,
            personality_interests: user.personality_interests.clone(),
            personality_interests_weight: user.personality_interests_weight,
            skills: user.skills.clone(),
            skills_weight: user.skills_weight,
            right_swipes: user.right_swipes.clone(),
            left_swipes: user.left_swipes.clone(),
            incoming_right_swipes: user.incoming_right_swipes.clone(),
            incoming_left_swipes: user.incoming_left_swipes.clone(),
            matches: user.matches.clone(),
        }
    }
}

// Handler for GET /users
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let outer_result = web::block(move || get_all_users(db)).await;
    match outer_result {
        Ok(inner_result) => 
        match inner_result {
            Ok(all_users) => Ok(HttpResponse::Ok().json(all_users)),
            Err(_) => Err(ServiceError::InternalServerError.into()),
        },
        Err(_) => Err(ServiceError::InternalServerError.into()),
    }
}

// Handler for GET /users/{id}
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: i32,
) -> Result<User, Error> {
    let user = web::block(move || db_get_user_by_id(db, user_id)).await;
    match user {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::InternalServerError.into()),
    }
}

// // Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    let outer_result = web::block(move || add_single_user(db, user)).await;
    match outer_result {
        Ok(inner_result) => 
        match inner_result {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Err(ServiceError::InternalServerError.into()),
        },
        Err(_) => Err(ServiceError::InternalServerError.into()),
    }
}

pub async fn update_user(
    db: web::Data<Pool>,
    user_id: i32,
    updated_user: InputUser,
) -> Result<HttpResponse, Error> {
    let outer_result = web::block(move || update_single_user(db, user_id, updated_user)).await;
    match outer_result {
        Ok(inner_result) => 
        match inner_result {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Err(ServiceError::InternalServerError.into()),
        },
        Err(_) => Err(ServiceError::InternalServerError.into()),
    }
}

// // Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: i32,
) -> Result<HttpResponse, Error> {
    let outer_result = web::block(move || delete_single_user(db, user_id)).await;
    match outer_result {
        Ok(inner_result) => 
        match inner_result {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Err(ServiceError::InternalServerError.into()),
        },
        Err(_) => Err(ServiceError::InternalServerError.into()),
    }
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> User {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn).expect("Error loading user")
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

fn add_single_user(
    db: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let input_user = user.into_inner();
    let new_user = NewUser {
        first_name: &input_user.first_name,
        last_name: &input_user.last_name,
        email: &input_user.email,
        created_at: chrono::Local::now().naive_local(),
        github: input_user.github.as_deref(),
        website: input_user.website.as_deref(),
        age: check_if_null(input_user.age.unwrap_or(0)),
        age_weight: check_if_null(input_user.age_weight.unwrap_or(0)),
        location: input_user.location.as_deref(),
        location_weight: check_if_null(input_user.location_weight.unwrap_or(0)),
        employer: input_user.employer.as_deref(),
        employer_weight: check_if_null(input_user.employer_weight.unwrap_or(0)),
        reason: input_user.reason.as_deref(),
        project_interests: input_user.project_interests.as_deref(),
        project_interests_weight: check_if_null(input_user.project_interests_weight.unwrap_or(0)),
        personality_interests: input_user.personality_interests.as_deref(),
        personality_interests_weight: check_if_null(input_user.personality_interests_weight.unwrap_or(0)),
        skills: input_user.skills.as_deref(),
        skills_weight: check_if_null(input_user.skills_weight.unwrap_or(0)),
        right_swipes: input_user.right_swipes,
        left_swipes: input_user.left_swipes,
        incoming_right_swipes: input_user.incoming_right_swipes,
        incoming_left_swipes: input_user.incoming_left_swipes,
        matches: input_user.matches,
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn update_single_user(
    db: web::Data<Pool>,
    user_id: i32,
    updated_user: InputUser,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let res: User = update(users.find(user_id))
        .set(
            (first_name.eq(&updated_user.first_name), 
            last_name.eq(&updated_user.last_name), 
            email.eq(&updated_user.email),
            github.eq(updated_user.github.as_deref()),
            website.eq(updated_user.website.as_deref()),
            age.eq(check_if_null(updated_user.age.unwrap_or(0))),
            age_weight.eq(check_if_null(updated_user.age_weight.unwrap_or(0))),
            location.eq(updated_user.location.as_deref()),
            location_weight.eq(check_if_null(updated_user.location_weight.unwrap_or(0))),
            employer.eq(updated_user.employer.as_deref()),
            employer_weight.eq(check_if_null(updated_user.employer_weight.unwrap_or(0))),
            reason.eq(updated_user.reason.as_deref()),
            project_interests.eq(updated_user.project_interests.as_deref()),
            project_interests_weight.eq(check_if_null(updated_user.project_interests_weight.unwrap_or(0))),
            personality_interests.eq(updated_user.personality_interests.as_deref()),
            personality_interests_weight.eq(check_if_null(updated_user.personality_interests_weight.unwrap_or(0))),
            skills.eq(updated_user.skills.as_deref()),
            skills_weight.eq(check_if_null(updated_user.skills_weight.unwrap_or(0))),
            right_swipes.eq(updated_user.right_swipes.as_deref()),
            left_swipes.eq(updated_user.left_swipes.as_deref()),
            incoming_right_swipes.eq(updated_user.incoming_right_swipes.as_deref()),
            incoming_left_swipes.eq(updated_user.incoming_left_swipes.as_deref()),
            matches.eq(updated_user.matches.as_deref()),  
        ))
        .get_result(&conn)?;
    Ok(res)
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}

fn check_if_null(input: i32) -> Option<i32> {
    match input {
        0 => None,
        _ => Some(input),
    }
}