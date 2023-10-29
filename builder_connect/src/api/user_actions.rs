use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo, api::auth};
use actix_web::{
    web::{Data, Path, Query},
    put,
    get,
    HttpResponse,
};
use std::env;
extern  crate dotenv;
use dotenv::dotenv;
use reqwest::{Response, header::HeaderValue};

#[put("/swipe_left/{id}")]
pub async fn swipe_left(db: Data<MongoRepo>, user_path: Path<String>, other_user_path: Path<String>) -> HttpResponse {
    let (user_id, other_user_id) =(user_path.into_inner(), other_user_path.into_inner());
    if user_id.is_empty() || other_user_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user = db.get_user(&user_id)
        .await
        .expect("DB error");

    let mut left_swipes = user.clone().left_swipes.unwrap();
    left_swipes.push(other_user_id);
    let updated_user = User{
        left_swipes: Some(left_swipes),
        ..user
    };

    let user_res = db.update_user(&user_id, updated_user).await;
    match user_res {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[put("/swipe_right/{id}")]
pub async fn swipe_right(db: Data<MongoRepo>, user_path: Path<String>, other_user_path: Path<String>) -> HttpResponse {
    let (user_id, other_user_id) =(user_path.into_inner(), other_user_path.into_inner());
    if user_id.is_empty() || other_user_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let mut user: User = db.get_user(&user_id)
        .await
        .expect("DB error");

    let mut other_user: User = db.get_user(&other_user_id)
        .await
        .expect("DB error");
    
    let mut user_right_swipes = user.clone().right_swipes.unwrap();
    user_right_swipes.push(other_user_id.clone());

    if match_exists(&other_user, &user_id) {
        let mut user_matches = user.matches.unwrap();
        user_matches.push(other_user_id.clone());
        let mut other_user_matches = other_user.matches.unwrap();
        other_user_matches.push(user_id.clone());
        user.matches = Some(user_matches);
        other_user.matches = Some(other_user_matches);
    }

    let user_res = db.update_user(&user_id, user.clone()).await;
    let other_user_res = db.update_user(&other_user_id, other_user.clone()).await;
    match (user_res, other_user_res) {
        (Ok(_), Ok(_)) => HttpResponse::Ok().body("Success"),
        (Err(_), Err(_)) => HttpResponse::InternalServerError().body("DB error"),
        _ => HttpResponse::InternalServerError().body("DB error"),
    }
}

fn match_exists(other_user: &User, user_id: &String) -> bool { 
    if other_user.right_swipes.as_ref().unwrap().contains(user_id){
        return true;
    }
    false
}

#[get("/matches/{id}")]
pub async fn view_matches(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user.matches),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[get("/recommend/{id}")]
// pub async fn recommend_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }
//     let user = db.get_user(&id).await.expect("DB Error");

//     // get top N users
//     let recommended_users = vec![];
//     let recommended_user = recommended_users[0];

//     // ensure they are not already matched + swiped on
//     Ok(HttpResponse::Ok().json(recommended_user))
// }

pub async fn generate_embedding(text: &String) -> Result<Vec<f32>, reqwest::Error>{
    dotenv().ok();
    let api_url = match env::var("HUGGING_FACE_API_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading API URL"),
    };
    let api_key = match env::var("HUGGING_FACE_API_KEY") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading API KEY"),
    };
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    let authorization_value = format!("Bearer {}", api_key);
    headers.insert("Authorization", HeaderValue::from_str(&authorization_value).unwrap());
    
    let response: Result<Response, reqwest::Error> = client
        .post(api_url)
        .headers(headers)
        .json(text)
        .send()
        .await;
    
    match response {
        Ok(response) => {
            let response_data: Vec<f32> = response.json().await?;
            Ok(response_data)
        },
        Err(_) => Err(reqwest::Error::from(response.unwrap_err())),
    }
}   