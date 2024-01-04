use oauth2::{
    AuthorizationCode,
    CsrfToken,
    PkceCodeChallenge,
    Scope,
    TokenResponse,
};
use std::env;
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};
use actix_web::{get, HttpResponse, web::{Data, Query}};
use actix_session::Session;
extern crate dotenv;
use reqwest;
use crate::{repository::mongodb_repo::MongoRepo, lib::OAuthClient};
use crate::lib::{ClientType, OAuthClientData};
#[derive(Debug, Deserialize, Clone)]
pub struct OAuthRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleClaims {
    pub sub: String,
    pub given_name: String,
    pub email: String,
    pub picture: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordClaims {
    pub id: String,
    pub username: String, 
    pub email: String,
    pub avatar: String,
}

#[get("/login")]
pub async fn login(
    client_type: Query<ClientType>, 
    data: Data<Vec<OAuthClientData>>,
    session: Session) -> HttpResponse {
    match validate(&session).await {
        Ok(res) => {
            if res {
                return HttpResponse::InternalServerError().json("Already Signed In");
            }
            else{
                // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
                let client: Data<OAuthClient>;
                let (auth_url, csrf_token);
                match client_type.into_inner() {
                    ClientType::DISCORD => {
                        client = data.iter().find(|&x| x.1 == ClientType::DISCORD).unwrap().0.clone();
                        (auth_url, csrf_token) = client
                            .authorize_url(CsrfToken::new_random)
                            .add_scope(Scope::new("email".to_string()))
                            .add_scope(Scope::new("identify".to_string()))
                            // .set_pkce_challenge(pkce_challenge)
                            .url();
                        
                    },
                    ClientType::GOOGLE => {
                        client = data.iter().find(|&x| x.1 == ClientType::GOOGLE).unwrap().0.clone();
                        (auth_url, csrf_token) = client
                            .authorize_url(CsrfToken::new_random)
                            .add_scope(Scope::new("profile".to_string()))
                            .add_scope(Scope::new("email".to_string()))
                            // .set_pkce_challenge(pkce_challenge)
                            .url();
                    }
                }
                return HttpResponse::Ok().json(auth_url.to_string());
            }
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn login_callback(
    db: Data<MongoRepo>,
    client_type: ClientType,
    client: Data<OAuthClient>,
    req: Query<OAuthRequest>,
    session: Session) -> HttpResponse {
        match validate(&session).await {
            Ok(res) => {
                if res {
                    return HttpResponse::InternalServerError().json("Already Signed In");
                }
                let token_result = client
                    .exchange_code(AuthorizationCode::new(req.into_inner().code))
                    // .set_pkce_verifier(PkceCodeVerifier::new(verifier.into_inner()))
                    .request_async(async_http_client)
                    .await.unwrap();
                let token_type = token_result.token_type();
                let access_token = token_result.access_token().secret();
                let url = match client_type {
                    ClientType::DISCORD => "https://discord.com/api/users/@me".to_string(),
                    ClientType::GOOGLE => format!("https://openidconnect.googleapis.com/v1/userinfo?alt=json&access_token={}", access_token),
                };
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::AUTHORIZATION, format!("{:?} {}", token_type, access_token).parse().unwrap());
                let res = reqwest::Client::new()
                    .get(url)
                    .headers(headers)
                    .send()
                    .await
                    .unwrap();
                let res_text = res.text().await.unwrap();
                let website_url: String = if in_production() {
                    env::var("PRODUCTION_URL").unwrap().to_string()
                }
                else{
                    env::var("LOCALHOST").unwrap().to_string()
                };
                match client_type {
                    ClientType::DISCORD => {
                        let claims: DiscordClaims = serde_json::from_str(&res_text).unwrap();
                        println!("{:?}", claims);
                        session.insert("sub_id", claims.id.clone()).unwrap();
                        let response_body = serde_json::to_string(&serde_json::json!({
                            "sub_id": format!("{}", claims.id.clone())
                        })).expect("Failed to serialize JSON");
                        let image_url  = format!("https://cdn.discordapp.com/avatars/{}/{}.png", claims.id.clone(), claims.avatar.clone());
                        return create_or_view_if_exists(
                            db, 
                            client_type, 
                            claims.id.clone(), 
                            claims.username.to_string(), 
                            claims.email.to_string(), 
                            image_url, 
                            website_url, 
                            response_body
                        ).await;
                    },
                    ClientType::GOOGLE => {
                        let claims: GoogleClaims = serde_json::from_str(&res_text).unwrap();
                        println!("{:?}", claims);
                        session.insert("sub_id", claims.sub.clone()).unwrap();
                        let response_body = serde_json::to_string(&serde_json::json!({
                            "sub_id": format!("{}", claims.sub.clone())
                        })).expect("Failed to serialize JSON");
                        return create_or_view_if_exists(
                            db, 
                            client_type, 
                            claims.sub.clone(), 
                            claims.given_name.to_string(), 
                            claims.email.to_string(), 
                            claims.picture.to_string(), 
                            website_url, 
                            response_body
                        ).await;
                    }
                }
            }
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        }
    } 

#[get("/login/callback/discord")]
pub async fn login_callback_discord(
    db: Data<MongoRepo>, 
    data: Data<Vec<OAuthClientData>>, 
    req: Query<OAuthRequest>, 
    session: Session) -> HttpResponse {
        let client: Data<OAuthClient> = data.iter().find(|&x| x.1 == ClientType::DISCORD).unwrap().0.clone();
        login_callback(db, ClientType::DISCORD, client, req, session).await
}

#[get("/login/callback/google")]
pub async fn login_callback_google(
    db: Data<MongoRepo>, 
    data: Data<Vec<OAuthClientData>>, 
    req: Query<OAuthRequest>, 
    session: Session) -> HttpResponse {
        let client: Data<OAuthClient> = data.iter().find(|&x| x.1 == ClientType::GOOGLE).unwrap().0.clone();
        login_callback(db, ClientType::GOOGLE, client, req, session).await
}

#[get("/get_session")]
pub async fn get_session(session: Session) -> HttpResponse {
    match session.get::<String>("sub_id") {
        Ok(message_option) => {
            match message_option {
                Some(message) => HttpResponse::Ok().json(message),
                None => HttpResponse::NotFound().body("Not set.")
            }
        }
	    Err(_) => HttpResponse::InternalServerError().body("Error.")
    }
}

#[get("/logout")]
pub async fn logout(session: Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok().json("Logged out") 
}

pub async fn validate(session: &Session) -> Result<bool, reqwest::Error> {
    if let Some(sub_id) = session.get::<String>("sub_id").unwrap() {
        let res = reqwest::get(format!("http://localhost:8080/view/{}", sub_id)).await;
        match res {
            Ok(_) => return Ok(true),
            Err(err) => return Err(err),
        }
    }
    Ok(false)
}

pub fn in_production() -> bool {
    match env::var("IN_PRODUCTION") {
        Ok(v) => {
            if v == "true" {
                return true;
            }
            false
        },
        Err(e) => {
            println!("Error checking production: {}", e.to_string());
            return false;
        },
    }
}

async fn create_or_view_if_exists(
    db: Data<MongoRepo>, 
    client_type: ClientType,
    sub_id: String, 
    username: String,
    email: String,
    image_url: String,
    website_url: String, 
    response_body: String) -> HttpResponse {
    match reqwest::get(format!("http://localhost:8080/view/{}", sub_id)).await {
        Ok(res) => {
            // Check if user exists in database
            if res.status() == 200 {
                direct_user_to_profile(website_url, response_body)
            }
            else{
                match db
                    .create_user(sub_id, username.clone(), email, if client_type == ClientType::DISCORD {username} else {"".to_string()}, image_url)
                    .await {
                        Ok(_) => direct_user_to_profile(website_url, response_body),
                        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), 
                    }
            }   
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    }
}

fn direct_user_to_profile(website_url: String, response_body: String) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", format!("{website_url}/profile/View")))
        .body(response_body)
}