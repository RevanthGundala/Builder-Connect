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
use dotenv::dotenv;
use crate::{GoogleOAuthClient, DiscordOAuthClient, ClientType};
use reqwest;
use crate::repository::mongodb_repo::MongoRepo;

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
    pub avatar: String,
    pub email: String,
}

#[get("/login")]
pub async fn login(
    client_type: Query<ClientType>, 
    google_data: Data<GoogleOAuthClient>,
    discord_data: Data<DiscordOAuthClient>, 
    session: Session) -> HttpResponse {
    match validate(&session).await {
        Ok(res) => {
            if res {
                return HttpResponse::InternalServerError().json("Already Signed In");
            }
            else{
                // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
                let (client, auth_url, csrf_token);
                match client_type.into_inner() {
                    ClientType::Google => {
                        client = google_data.client.clone();
                        (auth_url, csrf_token) = client
                            .authorize_url(CsrfToken::new_random)
                            .add_scope(Scope::new("profile".to_string()))
                            .add_scope(Scope::new("email".to_string()))
                            // .set_pkce_challenge(pkce_challenge)
                            .url();
                    }
                    ClientType::Discord => {
                        client = discord_data.client.clone();
                        (auth_url, csrf_token) = client
                            .authorize_url(CsrfToken::new_random)
                            .add_scope(Scope::new("email".to_string()))
                            .add_scope(Scope::new("identify".to_string()))
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

// async fn login_callback<T >(
//     db: Data<MongoRepo>,
//     data: Data<T>,
//     req: Query<OAuthRequest>,
//     session: Session)
//     where T : Clone {
//         match validate(&session).await {
//             Ok(res) => {
//                 if !res {
//                     let client = data.client.clone();
//                     let token_result = client
//                         .exchange_code(AuthorizationCode::new(req.into_inner().code))
//                         // .set_pkce_verifier(PkceCodeVerifier::new(verifier.into_inner()))
//                         .request_async(async_http_client)
//                         .await.unwrap();
//                     let url = "https://discord.com/api/users/@me";
//                     let token_type = token_result.token_type();
//                     let access_token = token_result.access_token().secret();
//                     let mut headers = reqwest::header::HeaderMap::new();
//                     headers.insert(reqwest::header::AUTHORIZATION, format!("{:?} {}", token_type, access_token).parse().unwrap());
//                     let res = reqwest::Client::new()
//                         .get(url)
//                         .headers(headers)
//                         .send()
//                         .await
//                         .unwrap();
//                     let res_text = res.text().await.unwrap();
//                     let claims: DiscordClaims = serde_json::from_str(&res_text).unwrap();
//                     println!("{:?}", claims);
//                     session.insert("sub_id", claims.id.clone()).expect("failed to insert sub_id into session");
//                     println!("Session: {:?}", session.get::<String>("sub_id").unwrap());
//                     let url = if in_production() {
//                         env::var("PRODUCTION_URL").unwrap().to_string()
//                     }
//                     else{
//                         env::var("LOCALHOST").unwrap().to_string()
//                     };
//                     let response_body = serde_json::to_string(&serde_json::json!({
//                         "sub_id": format!("{}", claims.id.clone())
//                     })).expect("Failed to serialize JSON");
//                     match reqwest::get(format!("http://localhost:8080/view/{}", claims.id.clone())).await {
//                         Ok(res) => {
//                             if res.status() != 200 {
//                                 match db.create_user(
//                                     claims.id.clone(), 
//                                     claims.username.to_string(), 
//                                     claims.email.to_string(), 
//                                     claims.username.to_string(), 
//                                     format!("https://cdn.discordapp.com/avatars/{}/{}.png", claims.id.clone(), claims.avatar)).await {
//                                         Ok(user) => {
//                                             HttpResponse::Found()
//                                                 .append_header(("Location", format!("{url}/profile/View")))
//                                                 .body(response_body)
//                                         },
//                                         Err(err) => HttpResponse::InternalServerError().body(err.to_string()), 
//                                 }
//                             }
//                             else{
//                                 HttpResponse::Found()
//                                     .append_header(("Location", format!("{url}/profile/View")))
//                                     .body(response_body)
                                
//                             }   
//                         }
//                         Err(err) => {
//                             return HttpResponse::InternalServerError().body(err.to_string());
//                         }
//                     }
//                 }
//                 else{
//                     return HttpResponse::Ok().json("/");
//                 }
//             }
//             Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
//         }

//     } 

#[get("/login/callback/discord")]
pub async fn login_callback_discord(
    db: Data<MongoRepo>, 
    discord_data: Data<DiscordOAuthClient>, 
    req: Query<OAuthRequest>, 
    session: Session) -> HttpResponse {
    match validate(&session).await {
        Ok(res) => {
            if !res {
                let client = discord_data.client.clone();
                let token_result = client
                    .exchange_code(AuthorizationCode::new(req.into_inner().code))
                    // .set_pkce_verifier(PkceCodeVerifier::new(verifier.into_inner()))
                    .request_async(async_http_client)
                    .await.unwrap();
                let url = "https://discord.com/api/users/@me";
                let token_type = token_result.token_type();
                let access_token = token_result.access_token().secret();
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::AUTHORIZATION, format!("{:?} {}", token_type, access_token).parse().unwrap());
                let res = reqwest::Client::new()
                    .get(url)
                    .headers(headers)
                    .send()
                    .await
                    .unwrap();
                let res_text = res.text().await.unwrap();
                let claims: DiscordClaims = serde_json::from_str(&res_text).unwrap();
                println!("{:?}", claims);
                session.insert("sub_id", claims.id.clone()).expect("failed to insert sub_id into session");
                println!("Session: {:?}", session.get::<String>("sub_id").unwrap());
                let url = if in_production() {
                    env::var("PRODUCTION_URL").unwrap().to_string()
                }
                else{
                    env::var("LOCALHOST").unwrap().to_string()
                };
                let response_body = serde_json::to_string(&serde_json::json!({
                    "sub_id": format!("{}", claims.id.clone())
                })).expect("Failed to serialize JSON");
                match reqwest::get(format!("http://localhost:8080/view/{}", claims.id.clone())).await {
                    Ok(res) => {
                        if res.status() != 200 {
                            match db.create_user(
                                claims.id.clone(), 
                                claims.username.to_string(), 
                                claims.email.to_string(), 
                                claims.username.to_string(), 
                                format!("https://cdn.discordapp.com/avatars/{}/{}.png", claims.id.clone(), claims.avatar)).await {
                                    Ok(user) => {
                                        HttpResponse::Found()
                                            .append_header(("Location", format!("{url}/profile/View")))
                                            .body(response_body)
                                    },
                                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()), 
                            }
                        }
                        else{
                            HttpResponse::Found()
                                .append_header(("Location", format!("{url}/profile/View")))
                                .body(response_body)
                            
                        }   
                    }
                    Err(err) => {
                        return HttpResponse::InternalServerError().body(err.to_string());
                    }
                }
            }
            else{
                return HttpResponse::Ok().json("/");
            }
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/login/callback/google")]
pub async fn login_callback_google(
    db: Data<MongoRepo>,
    google_data: Data<GoogleOAuthClient>, 
    req: Query<OAuthRequest>, 
    session: Session) -> HttpResponse {
    match validate(&session).await {
        Ok(res) => {
            if !res {
                let client = google_data.client.clone();
                let token_result = client
                    .exchange_code(AuthorizationCode::new(req.into_inner().code))
                    // .set_pkce_verifier(PkceCodeVerifier::new(verifier.into_inner()))
                    .request_async(async_http_client)
                    .await.unwrap();
                let url = format!("https://openidconnect.googleapis.com/v1/userinfo?alt=json&access_token={}", token_result.access_token().secret());
                let res = reqwest::get(&url).await.unwrap();
                let res_text = res.text().await.unwrap();
                let claims: GoogleClaims = serde_json::from_str(&res_text).unwrap();
                println!("{:?}", claims);
                session.insert("sub_id", claims.sub.clone()).expect("failed to insert sub_id into session");
                
                let url = if in_production() {
                    env::var("PRODUCTION_URL").unwrap().to_string()
                }
                else{
                    "http://localhost:3000".to_string()
                };
                match reqwest::get(format!("http://localhost:8080/view/{}", claims.sub.clone())).await {
                    Ok(res) => {
                        if res.status() != 200 {
                            match db.create_user(
                                claims.sub.clone(), 
                                claims.given_name.to_string(), 
                                claims.email.to_string(), 
                                "".to_string(), 
                                claims.picture).await {
                                    Ok(user) =>  return HttpResponse::Found().append_header(("Location", url)).finish(),
                                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()), 
                            }
                        }
                        else{
                            return HttpResponse::Found().append_header(("Location", url)).finish();
                        }   
                    }
                    Err(err) => {
                        return HttpResponse::InternalServerError().body(err.to_string());
                    }
                }
            }
            else{
                return HttpResponse::Ok().json("/");
            }
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }
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