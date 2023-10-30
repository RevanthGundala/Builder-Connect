mod api;
mod models;
mod repository;
use crate::api::user_actions::*;

//modify imports below
use actix_web::{web::Data, App, HttpServer};
use api::{user_api::*, auth::*};
use repository::mongodb_repo::MongoRepo;
use oauth2::{basic::BasicClient,
    AuthUrl,
    ClientId,
    ClientSecret,
    RedirectUrl,
    TokenUrl,
};

pub struct OAuthClient {
    client: BasicClient,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    let [client_id, client_secret, auth_url, token_url, redirect_url] = load_env_variables();
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()))
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());
    let oauth_client = OAuthClient { client };
    let oauth_client_data = Data::new(oauth_client);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(oauth_client_data.clone())
            .service(create_profile)
            .service(view_profile)
            .service(view_all_profiles)
            .service(edit_profile) 
            .service(delete_profile) 
            .service(swipe_left)
            .service(swipe_right)
            .service(recommend_user)
            .service(view_matches)
            .service(login)
            .service(login_callback)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}