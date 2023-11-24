mod api;
mod models;
mod repository;
mod chat;
use serde::{Deserialize, Serialize};
use actix_web::cookie::{ SameSite };
use actix_session::{ SessionMiddleware, Session };
use actix_session::config::{ BrowserSession, CookieContentSecurity };
use actix_session::storage::{ RedisActorSessionStore };
use actix_web::{web::Data, App, HttpServer, http, cookie::Key};
use api::{user_api::*, auth::*, user_actions::*};
use chat::socket::ChatServer;
use repository::mongodb_repo::MongoRepo;
use oauth2::{basic::BasicClient,
    AuthUrl,
    ClientId,
    ClientSecret,
    RedirectUrl,
    TokenUrl,
};
use actix_cors::Cors;
use actix::Actor;
pub struct GoogleOAuthClient {
    client: BasicClient,
}

pub struct DiscordOAuthClient{
    client: BasicClient,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")] 
#[serde(tag = "client_type", content = "value")]
pub enum ClientType{
    Google,
    Discord
}

impl ClientType{
    pub fn new_google_data() -> Data<GoogleOAuthClient> {
        let [client_id, client_secret, auth_url, token_url, redirect_url] = load_google_env_variables();
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).unwrap(),
            Some(TokenUrl::new(token_url).unwrap()))
            .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());
        let oauth_client = GoogleOAuthClient { client };
        let oauth_client_data = Data::new(oauth_client);
        oauth_client_data
    }

    pub fn new_discord_data() -> Data<DiscordOAuthClient> {
        let [client_id, client_secret, auth_url, token_url, redirect_url] = load_discord_env_variables();
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).unwrap(),
            Some(TokenUrl::new(token_url).unwrap()))
            .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());
        let oauth_client = DiscordOAuthClient { client };
        let oauth_client_data = Data::new(oauth_client);
        oauth_client_data
    }
}

struct OAuthClientData {
    google_client_data: Data<GoogleOAuthClient>,
    discord_client_data: Data<DiscordOAuthClient>,
}

fn get_client_data() -> OAuthClientData {
    OAuthClientData { 
        google_client_data: ClientType::new_google_data(), 
        discord_client_data: ClientType::new_discord_data() 
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    let OAuthClientData{
        google_client_data,
        discord_client_data,
    } = get_client_data();
    let signing_key = Key::generate(); 
    let chat_server =  ChatServer::new().start();
    let chat_server_data = Data::new(chat_server);
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:3000")
        //     .allowed_origin("http://localhost:8080")
        //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);
        let cors = Cors::permissive();
        App::new()
            .app_data(db_data.clone())
            .app_data(google_client_data.clone())
            .app_data(discord_client_data.clone())
            .app_data(chat_server_data.clone())
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    signing_key.clone(),
                )
                // allow the cookie to be accessed from javascript
                .cookie_http_only(false)
                // allow the cookie only from the current domain
                .cookie_same_site(SameSite::Strict)
                .build(),
            )
            .service(view_profile)
            .service(view_all_profiles)
            .service(edit_profile) 
            .service(delete_profile) 
            .service(swipe_left)
            .service(swipe_right)
            .service(recommend_user)
            .service(view_matches)
            .service(login)
            .service(login_callback_google)
            .service(login_callback_discord)
            .service(logout)
            .service(get_session)
            .service(create_many_users) //TODO: delete when done testing
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}