use mongodb::Client;
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
use reqwest::Url;

#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    // pub state: String,
    pub code: String,
    // pub scope: String,
    // pub authuser: String,
    // pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleClaims {
    pub sub: String,
    // pub given_name: String,
    // pub family_name: String,
    // pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordClaims {
    pub id: String,
    pub avatar: String,
    // pub family_name: String,
    // pub email: String,
}


pub fn load_google_env_variables() -> [String; 5]{
    dotenv().ok();
    let client_id = match env::var("GOOGLE_OAUTH_CLIENT_ID") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client_secret = match env::var("GOOGLE_OAUTH_CLIENT_SECRET") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let auth_url = match env::var("GOOGLE_OAUTH_AUTH_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let token_url = match env::var("GOOGLE_OAUTH_TOKEN_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let redirect_url = match env::var("GOOGLE_OAUTH_REDIRECT_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    [client_id, client_secret, auth_url, token_url, redirect_url]
}

pub fn load_discord_env_variables() -> [String; 5]{
    dotenv().ok();
    let client_id = match env::var("DISCORD_OAUTH_CLIENT_ID") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client_secret = match env::var("DISCORD_OAUTH_CLIENT_SECRET") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let auth_url = match env::var("DISCORD_OAUTH_AUTH_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let token_url = match env::var("DISCORD_OAUTH_TOKEN_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let redirect_url = match env::var("DISCORD_OUATH_REDIRECT_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    [client_id, client_secret, auth_url, token_url, redirect_url]
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
                return HttpResponse::Ok().json("/");
            }
            else{
                let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
                match client_type.into_inner() {
                    ClientType::Google => {
                        let client = google_data.client.clone();
                        let (auth_url, csrf_token) = client
                            .authorize_url(CsrfToken::new_random)
                            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
                            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
                            // .set_pkce_challenge(pkce_challenge)
                            .url();
                        return HttpResponse::Ok().json(auth_url.to_string());
                    }
                    ClientType::Discord => {
                        let client = discord_data.client.clone();
                        let (auth_url, csrf_token) = client
                            .authorize_url(CsrfToken::new_random)
                            .add_scope(Scope::new("email".to_string()))
                            .add_scope(Scope::new("identify".to_string()))
                            // .set_pkce_challenge(pkce_challenge)
                            .url();
                        return HttpResponse::Ok().json(auth_url.to_string());
                    }
                }
                
            }
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }
}
/*
 let client = discord_data.client.clone();
                        let token_result = client
                            .exchange_code(AuthorizationCode::new(req.into_inner().code))
                            .request_async(async_http_client)
                            .await
                            .unwrap();
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
                        println!("{:?}", res_text);
                        let claims: DiscordClaims = serde_json::from_str(&res_text).unwrap();
                        session.insert("sub_id", claims.id.clone()).unwrap();
                        let _ = reqwest::get(format!("http://localhost:8080/view/{}", claims.id.clone())).await.expect("Error"); */
// TODO: Add discord name and email and image url to the database
#[get("/login/callback/google")]
pub async fn login_callback(
    google_data: Data<GoogleOAuthClient>, 
    discord_data: Data<DiscordOAuthClient>,
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
                session.insert("sub_id", claims.sub.clone()).unwrap();
                let _ = reqwest::get(format!("http://localhost:8080/view/{}", claims.sub.clone())).await.expect("Error");
            }
            let url = format!("http://localhost:3000");
            return HttpResponse::Found().header("Location", url).finish();
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
            Ok(r) => return Ok(true),
            Err(err) => return Err(err),
        }
    }
    Ok(false)
}
