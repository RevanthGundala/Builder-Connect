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
    pub sub: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
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
pub async fn login(data: Data<OAuthClient>, session: Session) -> HttpResponse {
    println!("session: {:?}", session.entries());
    if let Some(sub_id) = session.get::<String>("sub_id").unwrap() {
        println!("Sucess");
        // let res = reqwest::get(format!("http://localhost:8080/view/{}", sub_id)).await;
        // match res {
        //     Ok(r) => return HttpResponse::Ok().json("Working"),
        //     Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        // }
    }
    let client = data.client.clone();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        // .set_pkce_challenge(pkce_challenge)
        .url();
    HttpResponse::Ok().json(auth_url.to_string())
}

#[get("/get_session")]
async fn get_session(session: Session) -> HttpResponse {
    match session.get::<String>("sub_id") {
	Ok(message_option) => {
	    match message_option {
		Some(message) => HttpResponse::Ok().body(message),
		None => HttpResponse::NotFound().body("Not set.")
	    }
	}
	Err(_) => HttpResponse::InternalServerError().body("Error.")
    }
}

#[get("/login/callback")]
pub async fn login_callback(data: Data<OAuthClient>, req: Query<OAuthRequest>, session: Session) -> HttpResponse {
    let client = data.client.clone();
    let token_result = client
        .exchange_code(AuthorizationCode::new(req.into_inner().code))
        // .set_pkce_verifier(PkceCodeVerifier::new(verifier.into_inner()))
        .request_async(async_http_client)
        .await.unwrap();

    let url = format!("https://openidconnect.googleapis.com/v1/userinfo?alt=json&access_token={}", token_result.access_token().secret());
    let res = reqwest::get(&url).await.unwrap();
    let res_text = res.text().await.unwrap();
    let claims: Claims = serde_json::from_str(&res_text).unwrap();
    session.insert("sub_id", claims.sub.clone()).unwrap();
    HttpResponse::Ok().json(claims.sub.clone())
    // let res = reqwest::get(format!("http://localhost:8080/view/{}", claims.sub)).await;
    // match res {
    //     Ok(r) => {
    //         session.insert("sub_id", claims.sub).unwrap();
    //         println!("session: {:?}", session.entries());
    //         HttpResponse::Ok().json(r.text().await.unwrap())
    //     },
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
}

#[get("/logout")]
pub async fn logout(session: Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok().json("Logged out") 
}
