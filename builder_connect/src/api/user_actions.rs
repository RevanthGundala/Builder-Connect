use crate::{models::user_model::{User, VectorEmbedding}, repository::mongodb_repo::MongoRepo};
use actix_web::{
    web::{Data, Path},
    put,
    get,
    HttpResponse,
};
use std::env;
extern crate dotenv;
use dotenv::dotenv;
use reqwest::{Response, header::HeaderValue};
use mongodb::{
    bson::{self, extjson::de::Error, doc, Document},
    results::InsertOneResult,
    Client, Collection, bson::Bson
};

#[put("/swipe_left/{sub_id}")]
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

    let res = db.update_user(&user_id, updated_user).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(_) => HttpResponse::InternalServerError().body("DB error"),
    }
}

#[put("/swipe_right/{sub_id}")]
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

#[get("/matches/{sub_id}")]
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

fn get_users_that_cannot_match(user: &User) -> Vec<String> {
    let mut users_cannot_match = vec![vec![user.sub_id.clone().unwrap()]];
    if let Some(left_swipes) = &user.left_swipes {
        users_cannot_match.push(left_swipes.to_owned());
    }
    if let Some(right_swipes) = &user.right_swipes {
        users_cannot_match.push(right_swipes.to_owned());
    }
    if let Some(matches) = &user.matches {
        users_cannot_match.push(matches.to_owned());
    }
    users_cannot_match.into_iter().flatten().collect()
}

#[get("/recommend/{sub_id}")]
pub async fn recommend_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user = db.get_user(&id).await.expect("DB Error");

    // get the the top 5 users with the highest cosine similarity
    // ensure they are not already matched + swiped on
    let users_cannot_match = get_users_that_cannot_match(&user);
    if let Some(embeddings) = &user.vector_embeddings {
        if embeddings.len() == 0 {
            return HttpResponse::Ok().body("No embeddings found");
        } 
        let mut recommendations = db.col.aggregate(
            vec![
                doc! {
                    "$vectorSearch": {
                        "index": "BuilderConnectSearch",
                        "path": "vector_embeddings",
                        "queryVector": bson::to_bson(&embeddings).unwrap(),
                        "numCandidates": 100,
                        "limit": 5,
                        
                    }
                },
            ],
            None,
        )
        .await
        .expect("Recommendation Error");
        
        // TODO: Figure out rankings
       while recommendations.advance().await.expect("Recommendation Error Loop") {
            let recommended_user: User = bson::from_document(recommendations.deserialize_current().unwrap()).unwrap();
            if !users_cannot_match.contains(&recommended_user.sub_id.clone().unwrap()) {
                return HttpResponse::Ok().json(recommended_user);
            }
        }
    }
    else{
        return HttpResponse::Ok().body("No embeddings found");
    }

    HttpResponse::Ok().json("Need to fetch more users")
}

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