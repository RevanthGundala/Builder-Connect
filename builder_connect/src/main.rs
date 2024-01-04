mod api;
mod models;
mod repository;
mod chat;
mod lib;
use api::email_api::{add_to_mailing_list, delete_from_mailing_list, send_email};
use lib::{get_client_data, ClientType};
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::{web::Data, App, HttpServer, cookie::{Key, SameSite}};
use api::{user_api::*, auth::*, user_actions::*, chat_api::*};
use crate::api::chat_api::get_conversation_by_id;
use chat::socket::ChatServer;
use repository::mongodb_repo::MongoRepo;
use actix_cors::Cors;
use actix::Actor;
extern crate dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    let signing_key = Key::generate(); 
    let chat_server =  ChatServer::new().start();
    let chat_server_data = Data::new(chat_server);
    let client_types: Vec<ClientType> = vec![ClientType::GOOGLE, ClientType::DISCORD];
    let client_data = Data::new(get_client_data(client_types));
    let redis_conn_string  = if in_production() {
        "redis:6379".to_string()
    }
    else{
        "127.0.0.1:6379".to_string()
    };
    
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:3000")
        //     .allowed_origin("http://localhost:8080")
        //     .allowed_origin("https://builder-connect.vercel.app")
        //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .supports_credentials()
        //     .max_age(3600);
        let cors = Cors::permissive();
        App::new()
            .app_data(db_data.clone())
            .app_data(chat_server_data.clone())
            .app_data(client_data.clone())
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(redis_conn_string.clone()),
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
            // .service(login_callback_google)
            // .service(login_callback_discord)
            .service(logout)
            .service(get_session)
            .service(create_many_users) //TODO: delete when done testing
            .service(delete_users)
            .service(delete_messages)
            .service(get_conversation_by_id)
            .service(start_chat_server)
            .service(add_to_mailing_list)
            .service(delete_from_mailing_list)
            .service(send_email)

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}