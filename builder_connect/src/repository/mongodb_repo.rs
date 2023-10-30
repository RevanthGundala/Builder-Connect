use std::env;
extern crate dotenv;
use actix_web::body::MessageBody;
use dotenv::dotenv;
use mongodb::{
    bson::{self, extjson::de::Error, doc},
    results::InsertOneResult,
    Client, Collection,
};
use crate::{models::user_model::{User, Time}, api::auth::Claims};
use mongodb::bson::oid::ObjectId;
use mongodb::results::{UpdateResult, DeleteResult};
use mongodb::IndexModel;
use mongodb::bson::extjson::de::Error::DeserializationError;

pub struct MongoRepo {
    col: Collection<User>,
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
        let col: Collection<User> = db.collection("Users");
        let index_model = IndexModel::builder()
            .keys(doc! {"sub_id": 1})
            .options(None)
            .build();
        let res = col.create_index(index_model, None).await.unwrap();
        if res.index_name != "sub_id_1" {
            panic!("PANIC!! Error creating index");
        }
        MongoRepo { col }
    }

    pub async fn create_user(&self, sub_id: String) -> Result<InsertOneResult, Error> {
        let new_user = User::new(sub_id);
        let res = self
            .col
            .insert_one(new_user, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(res)
    }

    pub async fn get_user(&self, sub_id: &String) -> Result<User, Error> {
        let filter = doc! {"sub_id": sub_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        match user_detail {
            Some(user) => Ok(user),
            None => Err(DeserializationError {message: "No user found".to_string()}),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursor = self
            .col
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
                    "first_name": new_user.first_name,
                    "last_name": new_user.last_name,
                    "email": new_user.email,
                    "github": new_user.github,
                    "website": new_user.website,
                    "age": new_user.age,
                    "location": new_user.location,  
                    "employer": new_user.employer,
                    "reason": new_user.reason,
                    "personality_interests": new_user.personality_interests,
                    "skills": new_user.skills,
                    "right_swipes": new_user.right_swipes,
                    "left_swipes": new_user.left_swipes,
                    "matches": new_user.matches,
                    "public_fields": bson::to_bson(&new_user.public_fields).unwrap(),
                    "vector_embeddings": bson::to_bson(&new_user.vector_embeddings).unwrap()
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, sub_id: &String) -> Result<DeleteResult, Error> {
        let filter = doc! {"sub_id": sub_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
}