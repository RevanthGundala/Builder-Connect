use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::{self, extjson::de::Error, doc},
    results::InsertOneResult,
    Client, Collection,
};
use crate::models::user_model::User;
use crate::models::message_model::Message;
use mongodb::results::{UpdateResult, DeleteResult};
use mongodb::IndexModel;
use mongodb::bson::extjson::de::Error::DeserializationError;

pub struct MongoRepo {
    pub users: Collection<User>,
    pub messages: Collection<Message>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("BuilderConnectDB");
        let users: Collection<User> = db.collection("Users");
        let messages: Collection<Message> = db.collection("Messages");
        let index_model = IndexModel::builder()
            .keys(doc! {"sub_id": 1})
            .options(None)
            .build();
        let res = users.create_index(index_model, None).await.unwrap();
        if res.index_name != "sub_id_1" {
            panic!("PANIC!! Error creating index");
        }
        MongoRepo { users, messages }
    }

    pub async fn create_user(
        &self, 
        sub_id: String, 
        username: String, 
        email: String, 
        discord: String, 
        image_url: String) -> Result<InsertOneResult, Error> {
        let new_user = User::new(sub_id);
        let new_user = User {
            username,
            email,
            discord,
            image_url,
            ..new_user
        };
        let res = self
            .users
            .insert_one(new_user, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(res)
    }

    pub async fn get_user(&self, sub_id: &String) -> Result<User, Error> {
        let filter = doc! {"sub_id": sub_id};
        let user_detail = self
            .users
            .find_one(filter, None)
            .await;
        match user_detail {
            Ok(user) => {
                match user {
                    Some(user) => Ok(user),
                    None => Err(DeserializationError {message: "No user found".to_string()}),
                }
            }
            Err(_) => Err(DeserializationError {message: "No user found".to_string()}),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursor = self
            .users
            .find(None, None)
            .await
            .ok()
            .expect("Error getting all users");
        let mut users = vec![];
        while cursor.advance().await.expect("Error getting all users") {
            users.push(cursor.deserialize_current().expect("Deserialization error"));
        }
        Ok(users)
    }

    pub async fn update_user(&self, sub_id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let filter = doc! {"sub_id": sub_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "sub_id": new_user.sub_id,
                    "image_url": new_user.image_url,
                    "username": new_user.username,
                    "email": new_user.email,
                    "discord": new_user.discord, 
                    "website": new_user.website,
                    "age": new_user.age,
                    "location": new_user.location,  
                    "employer": new_user.employer,
                    "reason": new_user.reason,
                    "project_interests": new_user.project_interests,
                    "personality_interests": new_user.personality_interests,
                    "skills": new_user.skills,
                    "right_swipes": new_user.right_swipes,
                    "left_swipes": new_user.left_swipes,
                    "matches": bson::to_bson(&new_user.matches).unwrap(),
                    "cannot_match": bson::to_bson(&new_user.cannot_match).unwrap(),
                    "public_fields": bson::to_bson(&new_user.public_fields).unwrap(),
                    "vector_embeddings": bson::to_bson(&new_user.vector_embeddings).unwrap(),
                    "last_seen": bson::to_bson(&new_user.last_seen).unwrap(),
                },
        };
        let updated_doc = self
            .users
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, sub_id: &String) -> Result<DeleteResult, Error> {
        let filter = doc! {"sub_id": sub_id};
        let user_detail = self
            .users
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    // ------------------- Messages ------------------- //

    pub async fn create_message(&self, message: Message) -> Result<InsertOneResult, Error> {
        let res = self
            .messages
            .insert_one(message, None)
            .await
            .ok()
            .expect("Error creating message");
        Ok(res)
    } 

    pub async fn get_messages_by_room_id(&self, room_id: &String) -> Result<Vec<Message>, Error> {
        let filter = doc! {"room_id": room_id};
        let mut cursor = self
            .messages
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting all messages");
        let mut messages = vec![];
        while cursor.advance().await.expect("Error getting all messages") {
            messages.push(cursor.deserialize_current().expect("Deserialization error"));
        }
        Ok(messages)
    }
}