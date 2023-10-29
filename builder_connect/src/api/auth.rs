use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
    RequestTokenError, StandardTokenResponse,
    PkceCodeVerifier,
    StandardRevocableToken
};
use std::{env, any::Any};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use serde::{Deserialize, Serialize};
use actix_web::{get, HttpResponse, web::{Data, Path, Query}, rt::net::TcpListener};
extern crate dotenv;
use dotenv::dotenv;
use std::io::{BufReader, Write, BufRead};
use crate::OAuthClient;

#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    pub state: String,
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub email: String,
    pub name: String,
}

pub fn load_env_variables() -> [String; 5]{
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

#[get("/login")]
pub async fn login(data: Data<OAuthClient>) -> HttpResponse {
    let client = data.client.clone();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        // .set_pkce_challenge(pkce_challenge)
        .url();
    HttpResponse::Ok().body(auth_url.to_string())
}

#[get("/login/callback")]
pub async fn login_callback(data: Data<OAuthClient>, req: Query<OAuthRequest>) -> HttpResponse {
    let client = data.client.clone();
    let token_result = client
        .exchange_code(AuthorizationCode::new(req.into_inner().code))
        // .set_pkce_verifier(PkceCodeVerifier::new(verifier.into_inner()))
        .request_async(async_http_client)
        .await.unwrap();

    let url = format!("https://openidconnect.googleapis.com/v1/userinfo?alt=json&access_token={}", token_result.access_token().secret());
    let res = reqwest::get(&url).await.unwrap();
    println!("Sub: {}", res.text().await.unwrap());
    let sub = "";
    let res = reqwest::get(format!("/profile/{}", sub)).await.unwrap();

    HttpResponse::Ok().body(token_result.access_token().secret().to_string())
}
