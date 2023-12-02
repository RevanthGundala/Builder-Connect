use std::time::Instant;
use actix::*;
use actix_web::{get, post, web::{Data, Payload}, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use mongodb::bson::Uuid;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use rand::{rngs::ThreadRng, Rng};
use super::session;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: String,
    pub room_id: String,
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: String,
    pub room_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub user_id: String,
    pub room_id: String,
    pub content: String,
}

// pub struct ListRooms;

// impl actix::Message for ListRooms {
//     type Result = Vec<String>;
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Join {
//     pub id: usize,
//     pub name: String,
// }

#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<String, Recipient<Message>>,
    rooms: HashMap<String, HashSet<String>>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        Self {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn send_message(&self, message: &str, id_to: &String) {
        println!("sending message to {id_to}");
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(Message(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self
            .rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new).insert(msg.user_id.clone());
        self
            .sessions
            .insert(msg.user_id.clone(), msg.addr);

        self.send_message(&json!({
            "content": format!("{} connected!", msg.user_id),
            "chat_type": session::ChatType::CONNECT
        }).to_string(), &msg.user_id);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        if self.sessions.remove(&msg.user_id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.user_id)
                .for_each(|user_id| self.send_message(&json!({
                    "content": format!("{} disconnected!", msg.user_id),
                    "chat_type": session::ChatType::DISCONNECT}).to_string(), user_id));
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        self
            .rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|user_id| self.send_message(&msg.content, user_id));
    }
}

// impl Handler<ListRooms> for ChatServer {
//     type Result = MessageResult<ListRooms>;

//     fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
//         let mut rooms = vec![];
//         for key in self.rooms.keys() {
//             rooms.push(key.to_owned());
//         }
//         MessageResult(rooms)
//     }
// }

// impl Handler<Join> for ChatServer {
//     type Result = ();

//     fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
//         let Join {id, name} = msg;
//         let mut rooms = vec![];

//         for (n, sessions) in &mut self.rooms {
//             if sessions.remove(&id) {
//                 rooms.push(n.to_owned());
//             }
//         }

//         for room in rooms {
//             self.send_message(&room, &json!({
//                 "room": room,
//                 "value": vec![format!("Someone disconnect!")],
//                 "chat_type": session::ChatType::DISCONNECT
//             }).to_string(), 0);
//         }

//         self.rooms
//             .entry(name.clone())
//             .or_insert_with(HashSet::new)
//             .insert(id);
//     }
// }