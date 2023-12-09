use crate::{models::user_model::{User, Room}, repository::mongodb_repo::MongoRepo};
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
use mongodb::bson::{self, doc, Uuid};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[put("/swipe_left/{sub_id}/{other_sub_id}")]
pub async fn swipe_left(db: Data<MongoRepo>, path: Path<(String, String)>) -> HttpResponse {
    let (user_id, other_user_id) = path.into_inner();
    if user_id.is_empty() || other_user_id.is_empty() || user_id == other_user_id {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user = db.get_user(&user_id)
        .await
        .expect("DB error");

    let mut left_swipes = user.clone().left_swipes.unwrap();
    left_swipes.push(other_user_id.clone());

    let mut cannot_match = user.clone().cannot_match.unwrap();
    cannot_match.push(other_user_id);

    let updated_user = User{
        left_swipes: Some(left_swipes),
        cannot_match: Some(cannot_match),
        ..user
    };

    let res = db.update_user(&user_id, updated_user).await;
    match res {
        Ok(_) => HttpResponse::Ok().json("Swipe Left Success"),
        Err(_) => HttpResponse::InternalServerError().body("DB error"),
    }
}

#[put("/swipe_right/{sub_id}/{other_sub_id}")]
pub async fn swipe_right(db: Data<MongoRepo>, path: Path<(String, String)>) -> HttpResponse {
    let (user_id, other_user_id) = path.into_inner();
    if user_id.is_empty() || other_user_id.is_empty() || user_id == other_user_id {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user: User = db.get_user(&user_id)
        .await
        .expect("DB error");

    let mut other_user: User = db.get_user(&other_user_id)
        .await
        .expect("DB error");
    
    let mut user_right_swipes = user.right_swipes.unwrap();
    user_right_swipes.push(other_user_id.clone());

    let mut cannot_match = user.cannot_match.unwrap();
    cannot_match.push(other_user_id.clone());
    
    let mut updated_user = User{
        right_swipes: Some(user_right_swipes),
        cannot_match: Some(cannot_match),
        ..user
    };
    if match_exists(&other_user, &user_id) {
        let uuid = Uuid::new();
        let res = reqwest::Client::new()
            .get(format!("http://localhost:8080/chat/{uuid}"))
            .send()
            .await;
        match res {
            Ok(_) => {
                let mut user_matches = updated_user.matches.unwrap();
                user_matches.push(Room::new(uuid, other_user_id.clone()));
        
                let mut other_user_matches = other_user.matches.unwrap();
                other_user_matches.push(Room::new(uuid, user_id.clone()));
                
                updated_user = User{
                    matches: Some(user_matches),
                    ..updated_user
                };
                other_user = User{
                    matches: Some(other_user_matches),
                    ..other_user
                };
                
                let (updated_user_username, other_user_username) = (updated_user.username.clone(), other_user.username.clone());
                let (updated_user_email, other_user_email) = (updated_user.email.clone(), other_user.email.clone());
                let _ = send_email(
                    updated_user_email,
                    "You have a new match!".to_string(),
                    format!("You have a new match with {other_user_username}! You can now chat with them at http://localhost:8080"),
                ).await.expect("Email error");
                let _ = send_email(
                    other_user_email,
                    "You have a new match!".to_string(),
                    format!("You have a new match with {updated_user_username}! You can now chat with them at http://localhost:8080"),
                ).await.expect("Email error");
            },
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    let user_res = db.update_user(&user_id, updated_user.clone()).await;
    let other_user_res = db.update_user(&other_user_id, other_user.clone()).await;
    match (user_res, other_user_res) {
        (Ok(_), Ok(_)) => HttpResponse::Ok().json("Swipe Right Success"),
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


#[get("/recommend/{sub_id}")]
pub async fn recommend_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user = db.get_user(&id).await.expect("DB Error");

    // get the the top 5 users with the highest cosine similarity
    // ensure they are not already matched + swiped on
    let users_cannot_match = user.cannot_match.unwrap();
    if let Some(embeddings) = &user.vector_embeddings {
        if embeddings.len() == 0 {
            return HttpResponse::Ok().body("No embeddings found");
        } 
        let mut recommendations = db.users.aggregate(
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

pub async fn send_email(
    to: String,
    subject: String,
    body: String) -> Result<(), reqwest::Error> { 
    let email_username = match env::var("BUILDER_CONNECT_EMAIL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading email username"),
    };
    let email = Message::builder()
        .from(format!("<{email_username}>").parse().unwrap())
        .to(format!("<{to}>").parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();
    let email_password = match env::var("BUILDER_CONNECT_EMAIL_PASSWORD") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading email password"),
    };

    let creds = Credentials::new(email_username, email_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
    Ok(())
}