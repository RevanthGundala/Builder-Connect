use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::prelude::*;
use crate::errors::ServiceError;
use actix_web::{web, Error, HttpResponse};
use argonautica::input;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

use crate::handlers::*;
// user logs in (TODO: AUTH)

// user creates profile (handler calls create_user)

// user is matched with other user
// TODO: matching algo

// can swipe left or right on other user
pub async fn swipe_left(db: web::Data<Pool>, sender_id: i32, other_user_id: i32) -> Result<HttpResponse, Error> {
    let sender = get_user_by_id(db, sender_id).await.unwrap();
    let updated_sender = InputUser {
        left_swipes: Some(sender.left_swipes.unwrap().push(other_user_id)),
        ..sender
    };
    let other_user = get_user_by_id(db, other_user_id).await.unwrap();
    let updated_other_user = InputUser {
        incoming_left_swipes: Some(other_user.incoming_left_swipes.unwrap().push(sender_id)),
        ..other_user
    };
    let sender_res = update_user(db, sender_id, sender).await;
    let other_user_res = update_user(db, other_user_id, other_user).await;
    (sender_res, other_user_res)
}

// if both users swipe right, they are matched


// user can view their matches

// user can view their profile and other people's profiles

// user can edit their profile

// user can delete their profile

// user can log out