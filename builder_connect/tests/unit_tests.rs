use builder_connect;
use actix_web::{http::StatusCode, test, App};
use diesel::prelude::*;

use serde_json::json;
use crate::models::*;
use super::schema::users::dsl::*;
use super::Pool;
use crate::errors::ServiceError;
use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse};
use argonautica::input;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::sync::Arc;

#[actix_rt::test]
async fn test_swipe_left() {
    let mut app = test::init_service(
        App::new()
            .data_pool()
            .route("/swipe_left/{sender_id}/{other_user_id}", web::post().to(swipe_left)),
    )
    .await;

    // Insert two users into the database
    let conn = &app.data().get().unwrap().get().unwrap();
    let new_user1 = NewUser {
        name: "User1".to_string(),
        email: "user1@example.com".to_string(),
        password: "password".to_string(),
        ..Default::default()
    };
    let new_user2 = NewUser {
        name: "User2".to_string(),
        email: "user2@example.com".to_string(),
        password: "password".to_string(),
        ..Default::default()
    };
    let user1: User = diesel::insert_into(users::table)
        .values(&new_user1)
        .get_result(conn)
        .unwrap();
    let user2: User = diesel::insert_into(users::table)
        .values(&new_user2)
        .get_result(conn)
        .unwrap();

    // Send a POST request to swipe left on user2
    let req = test::TestRequest::post()
        .uri(&format!("/swipe_left/{}/{}", user1.id, user2.id))
        .to_request();
    let (res1, res2) = test::read_response_json::<(Json<OutputUser>, Json<OutputUser>)>(&mut app, req).await;

    // Check that the response is successful
    assert_eq!(res1.status(), StatusCode::OK);
    assert_eq!(res2.status(), StatusCode::OK);

    // Check that the swipes were recorded correctly
    let updated_user1: User = users::table.find(user1.id).first(conn).unwrap();
    let updated_user2: User = users::table.find(user2.id).first(conn).unwrap();
    assert_eq!(updated_user1.left_swipes.unwrap(), vec![user2.id]);
    assert_eq!(updated_user2.incoming_left_swipes.unwrap(), vec![user1.id]);
}

#[actix_rt::test]
async fn test_swipe_right() {
    let mut app = test::init_service(
        App::new()
            .data_pool()
            .route("/swipe_right/{sender_id}/{other_user_id}", web::post().to(swipe_right)),
    )
    .await;

    // Insert two users into the database
    let conn = &app.data().get().unwrap().get().unwrap();
    let new_user1 = NewUser {
        name: "User1".to_string(),
        email: "user1@example.com".to_string(),
        password: "password".to_string(),
        ..Default::default()
    };
    let new_user2 = NewUser {
        name: "User2".to_string(),
        email: "user2@example.com".to_string(),
        password: "password".to_string(),
        ..Default::default()
    };
    let user1: User = diesel::insert_into(users::table)
        .values(&new_user1)
        .get_result(conn)
        .unwrap();
    let user2: User = diesel::insert_into(users::table)
        .values(&new_user2)
        .get_result(conn)
        .unwrap();

    // Send a POST request to swipe right on user2
    let req = test::TestRequest::post()
        .uri(&format!("/swipe_right/{}/{}", user1.id, user2.id))
        .to_request();
    let (res1, res2) = test::read_response_json::<(Json<OutputUser>, Json<OutputUser>)>(&mut app, req).await;

    // Check that the response is successful
    assert_eq!(res1.status(), StatusCode::OK);
    assert_eq!(res2.status(), StatusCode::OK);

    // Check that the swipes were recorded correctly
    let updated_user1: User = users::table.find(user1.id).first(conn).unwrap();
    let updated_user2: User = users::table.find(user2.id).first(conn).unwrap();
    assert_eq!(updated_user1.right_swipes.unwrap(), vec![user2.id]);
    assert_eq!(updated_user2.incoming_right_swipes.unwrap(), vec![user1.id]);
}

#[actix_rt::test]
async fn test_view_profile() {
    let mut app = test::init_service(
        App::new()
            .data_pool()
            .route("/view_profile/{user_id}", web::get().to(view_profile)),
    )
    .await;

    // Insert a user into the database
    let conn = &app.data().get().unwrap().get().unwrap();
    let new_user = NewUser {
        name: "User".to_string(),
        email: "user@example.com".to_string(),
        password: "password".to_string(),
        ..Default::default()
    };
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .unwrap();

    // Send a GET request to view the user's profile
    let req = test::TestRequest::get()
        .uri(&format!("/view_profile/{}", user.id))
        .to_request();
    let res = test::read_response_json::<Json<OutputUser>>(&mut app, req).await;

    // Check that the response is successful and contains the correct user data
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.into_inner(),
        json!({
            "id": user.id,
            "name": "User",
            "email": "user@example.com",
            "bio": None,
            "age": None,
            "gender": None,
            "location": None,
            "interests": None,
            "left_swipes": None,
            "right_swipes": None,
            "incoming_left_swipes": None,
            "incoming_right_swipes": None,
        })
    );
}

#[actix_rt::test]
async fn test_edit_profile() {
    let mut app = test::init_service(
        App::new()
            .data_pool()
            .route("/edit_profile/{user_id}", web::put().to(edit_profile)),
    )
    .await;

    // Insert a user into the database
    let conn = &app.data().get().unwrap().get().unwrap();
    let new_user = NewUser {
        name: "User".to_string(),
        email: "user@example.com".to_string(),
        password: "password".to_string(),
        ..Default::default()
    };
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .unwrap();

    // Send a PUT request to edit the user's profile
    let updated_user = InputUser {
        name: Some("New Name".to_string()),
        bio: Some("New Bio".to_string()),
        ..Default::default()
    };
    let req = test::TestRequest::put()
        .uri(&format!("/edit_profile/{}", user.id))
        .set_json(&updated_user)
        .to_request();
    let res = test::call_service(&mut app, req).await;

    // Check that the response is successful and the user's data was updated
    assert_eq!(res.status(), StatusCode::OK);
    let updated_user: User = users::table.find(user.id).first(conn).unwrap();
    assert_eq!(updated_user.name, "New Name");
    assert_eq!(updated_user.bio.unwrap(), "New Bio");
}

#[actix_rt::test]
async fn test_delete_profile() {
    let mut app = test::init_service(
        App::new()
            .data_pool()
            .route("/delete_profile/{user_id}", web::delete().to(delete_profile)),
    )
    .await;

    // Insert a user into the database
    let conn = &app.data().get().unwrap().get().unwrap();
    let new_user = NewUser {
        name: "User".to_string(),
        email: "user@example.com".to_string(),
        password: "password".to_string(),
        ..Default::default()
    };
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .unwrap();

    // Send a DELETE request to delete the user's profile
    let req = test::TestRequest::delete()
        .uri(&format!("/delete_profile/{}", user.id))
        .to_request();
    let res = test::call_service(&mut app, req).await;

    // Check that the response is successful and the user was deleted from the database
    assert_eq!(res.status(), StatusCode::OK);
    let deleted_user: Result<User, diesel::result::Error> = users::table.find(user.id).first(conn);
    assert!(deleted_user.is_err());
}


