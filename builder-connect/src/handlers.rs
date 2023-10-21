use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::prelude::*;
use crate::errors::ServiceError;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into, update};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
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
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let outer_result = web::block(move || db_get_user_by_id(db, user_id.into_inner())).await;
    match outer_result {
        Ok(inner_result) => 
        match inner_result {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Err(ServiceError::InternalServerError.into()),
        },
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
    user_id: web::Path<i32>,
    updated_user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    let outer_result = web::block(move || update_single_user(db, user_id.into_inner(), updated_user)).await;
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
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let outer_result = web::block(move || delete_single_user(db, user_id.into_inner())).await;
    match outer_result {
        Ok(inner_result) => 
        match inner_result {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Err(ServiceError::InternalServerError.into()),
        },
        Err(_) => Err(ServiceError::InternalServerError.into()),
    }
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
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
    let new_user = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn update_single_user(
    db: web::Data<Pool>,
    user_id: i32,
    updated_user: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let res: User = update(users.find(user_id))
        .set((first_name.eq(&updated_user.first_name), last_name.eq(&updated_user.last_name), email.eq(&updated_user.email)))
        .get_result(&conn)?;
    Ok(res)
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}