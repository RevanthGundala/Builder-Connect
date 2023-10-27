use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{self, extjson::de::Error, doc},
    results::{ InsertOneResult},
    Client, Collection,
};
use crate::models::user_model::{User, UserView};
use mongodb::bson::oid::ObjectId;
use mongodb::results::{UpdateResult, DeleteResult};

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
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let res = self
            .col
            .insert_one(new_user, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(res)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
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
            users.push(cursor.deserialize_current().unwrap());
        }
        Ok(users)
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
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
                    "incoming_right_swipes": new_user.incoming_right_swipes,
                    "incoming_left_swipes": new_user.incoming_left_swipes,
                    "matches": new_user.matches,
                    "public_fields": bson::to_bson(&new_user.public_fields).unwrap(),
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

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
}