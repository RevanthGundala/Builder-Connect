use std::time::Instant;
use actix::*;
use actix_web::{get, post, web::{self, Data, Path}, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use mongodb::bson::Uuid;
use serde_json::json;
use crate::{repository::mongodb_repo::MongoRepo, chat::session::WsChatSession};
use crate::chat::session;
use crate::chat::socket::ChatServer;

#[get("/chat/{room_id}")]
pub async fn start_chat_server(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<MongoRepo>,
    srv: Data<Addr<ChatServer>>,
    room_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let room_id = room_id.into_inner();
    println!("Room id: {:?}", room_id);
    let ws = WsChatSession::new(
        room_id,
        srv.get_ref().clone(),
        db.clone(),
    );
    let res = ws::start(ws, &req, stream)?;
    Ok(res)
}

#[get("/messages/{room_id}")]
pub async fn get_conversation_by_id(
    db: Data<MongoRepo>,
    room_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let room_id = room_id.into_inner();
    let conversations = db.get_messages_by_room_id(&room_id).await.expect("Messages by room id error");
    Ok(HttpResponse::Ok().json(conversations))
}