mod api;
mod models;
mod repository;
use crate::api::user_actions::*;

//modify imports below
use actix_web::{web::Data, App, HttpServer};
use api::user_api::*;
use repository::mongodb_repo::MongoRepo;
use std::sync::Arc;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    let model = Arc::new({
        SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()
        .expect("Error creating model")});
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(model.clone())
            .service(create_profile)
            .service(view_profile)
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