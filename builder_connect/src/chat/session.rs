use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web::web::{self, Data, block};
use actix_web_actors::ws;
use futures::FutureExt;
use mongodb::bson::{oid::ObjectId, Uuid};
use super::socket::{ChatServer, Connect, Disconnect, self, ClientMessage};
use crate::repository::mongodb_repo::MongoRepo;
use serde::{Deserialize, Serialize};
use crate::models::message_model::Message;
use chrono::{DateTime, Utc};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsChatSession {
    pub id: String,
    pub room_id: String,
    pub hb: Instant,
    pub addr: Addr<ChatServer>,
    pub db: Data<MongoRepo>,
}

impl WsChatSession {
    pub fn new(room_id: String, chat_server: Addr<ChatServer>, db: Data<MongoRepo>) -> Self {
        WsChatSession {
            id: Uuid::new().to_string(),
            room_id,
            hb: Instant::now(),
            addr: chat_server,
            db
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting, failed heartbeat");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum ChatType {
    STATUS,
    TYPING,
    TEXT,
    CONNECT,
    DISCONNECT,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    pub id: Option<Uuid>,
    pub room_id: String,
    pub user_id: String,
    pub chat_type: ChatType,
    pub content: String,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();

        self.addr
            .send(Connect {
                user_id: self.id.clone(),
                room_id: self.room_id.clone(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
}

impl Handler<socket::Message> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: socket::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let data_json = serde_json::from_str::<ChatMessage>(&text.to_string());
                if let Err(err) = data_json {
                    println!("{err}");
                    println!("Failed to parse message: {text}");
                    return;
                }

                let input = data_json.as_ref().unwrap();
                match &input.chat_type {
                    ChatType::TYPING => {
                        let chat_msg = ChatMessage {
                            chat_type: ChatType::TYPING,
                            content: input.content.to_string(),
                            id: Some(Uuid::parse_str(self.id.clone()).unwrap()),
                            room_id: input.room_id.clone(),
                            user_id: input.user_id.to_string(),
                        };
                        let msg = serde_json::to_string(&chat_msg).unwrap();
                        self.addr.do_send(ClientMessage {
                            user_id: self.id.clone(),
                            room_id: self.room_id.clone(),
                            content: msg
                        })
                    }
                    ChatType::TEXT => {
                        let db = self.db.clone();
                        let room_id_clone = self.room_id.clone();
                        let (input_room_id, input_user_id, input_content) = (
                            input.room_id.clone(),
                            input.user_id.clone(),
                            input.content.clone(),
                        );
                        let fut = async move {
                            // check if last message was sent within 5 minutes
                            // if so, don't revord timestamp

                            let mut new_message = Message::new(
                                input_room_id.clone(),
                                input_user_id.to_string(),
                                input_content.clone(),
                                None,
                            );
                            let messages = db.get_messages_by_room_id(&room_id_clone)
                                .await
                                .unwrap();
                            if let Some(last_msg) = messages.last() {
                                if let Some(last_msg_time) = last_msg.created_at {
                                    if last_msg_time < Utc::now() - chrono::Duration::minutes(5) {
                                        new_message = Message::new(
                                            input_room_id,
                                            input_user_id,
                                            input_content,
                                            Some(Utc::now()),
                                        );
                                    }
                                }
                            }
                            
                            let _ = db.create_message(new_message).await;
                        };
                        let fut = actix::fut::wrap_future::<_, Self>(fut);
                        ctx.spawn(fut);
                        let chat_msg = ChatMessage {
                            id: Some(Uuid::parse_str(self.id.clone()).unwrap()),
                            room_id: input.room_id.clone(),
                            user_id: input.user_id.to_string(),
                            chat_type: ChatType::TEXT,
                            content: input.content.to_string(),
                        };
                        let msg = serde_json::to_string(&chat_msg).unwrap();
                        self.addr.do_send(ClientMessage {
                            user_id: self.id.clone(),
                            room_id: self.room_id.clone(),
                            content: msg
                        })
                    }
                    _ => {}
                }
            }
            ws::Message::Binary(_) => println!("Unsupported binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}