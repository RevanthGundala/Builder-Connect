use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::prelude::*;
use crate::errors::ServiceError;
use actix_web::{web, Error, HttpResponse};
use argonautica::input;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::sync::Arc;

use crate::handlers::*;
// user logs in (TODO: AUTH)

// user creates profile (handler calls create_user)

// user is matched with other user
// TODO: matching algo

// can swipe left or right on other user
pub async fn swipe_left(db: &web::Data<Pool>, sender_id: web::Path<i32>, other_user_id: web::Path<i32>) -> (Result<HttpResponse, Error>, Result<HttpResponse, Error>) {
    let (sender_id, other_user_id) = (sender_id.into_inner(), other_user_id.into_inner());

    let mut sender = get_user_by_id(db.clone(), sender_id)
        .await
        .expect("DB error");
    let mut other_user = get_user_by_id(db.clone(), other_user_id)
        .await
        .expect("DB error");

    sender.left_swipes = sender.left_swipes.map(|mut swipes| {
        swipes.push(other_user_id);
        swipes
    });
    other_user.incoming_left_swipes = other_user.incoming_left_swipes.map(|mut swipes| {
        swipes.push(sender_id);
        swipes
    });
    
    let updated_sender = InputUser {
        ..sender.into()
    };
    let updated_other_user = InputUser {
        ..other_user.into()
    };
    let sender_res = update_user(db.clone(), sender_id, updated_sender).await;
    let other_user_res = update_user(db.clone(), other_user_id, updated_other_user).await;
    (sender_res, other_user_res)
}

// pub async fn swipe_right(db: web::Data<Pool>, sender_id: i32, other_user_id: i32) -> Result<HttpResponse, Error> {
//     let sender = get_user_by_id(db, sender_id).await.unwrap();
//     let updated_sender = InputUser {
//         right_swipes: Some(sender.right_swipes.unwrap().push(other_user_id)),
//         ..sender
//     };
//     let other_user = get_user_by_id(db, other_user_id).await.unwrap();
//     let updated_other_user = InputUser {
//         incoming_right_swipes: Some(other_user.incoming_right_swipes.unwrap().push(sender_id)),
//         ..other_user
//     };
//     let sender_res = update_user(db, sender_id, sender).await;
//     let other_user_res = update_user(db, other_user_id, other_user).await;
//     (sender_res, other_user_res)
// }

// user can view their profile and other people's profiles
// pub async fn view_profile(db: web::Data<Pool>, user_id: i32) -> Result<HttpResponse, Error> {
//     let res = get_user_by_id(db, user_id).await.unwrap();
//     res
// }

// user can edit their profile

// user can delete their profile

// user can log out






fn update_user_left_swipes() {

}