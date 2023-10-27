mod api;
mod models;
mod repository;
use crate::api::user_actions::*;

//modify imports below
use actix_web::{web::Data, App, HttpServer};
use api::user_api::*;
use repository::mongodb_repo::MongoRepo;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_profile)
            .service(view_profile)
            .service(view_all_profiles)
            .service(edit_profile) 
            .service(delete_profile) 
            .service(swipe_left)
            .service(swipe_right)
            .service(view_matches)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}