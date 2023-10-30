use crate::{models::user_model::{User, VectorEmbedding}, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post,
    get,
    put,
    delete,
    web::{Data, Json, Path, Query},
    HttpResponse, cookie::time::Date,
};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::extjson::de::Error as MongoError;
use reqwest::Client;
use super::user_actions::generate_embedding;
use chrono::{DateTime, Utc};
use crate::models::user_model::Time;
use crate::api::auth::Claims;
use crate::api::user_api;

#[post("/create/{sub_id}")]
pub async fn create_profile(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let sub_id = path.into_inner();
    if sub_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    } 
    let user_detail = db.create_user(sub_id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()), 
    }
}

#[get("/view/{sub_id}")]
pub async fn view_profile(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let sub_id = path.into_inner();
    if sub_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid sub_id");
    }
    let user_detail = db.get_user(&sub_id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(MongoError) => {
            let client = Client::new();
            let res = client.post(format!("http://localhost:8080/create/{}", sub_id))
                .send()
                .await
                .unwrap();
            HttpResponse::Ok().json(res.text().await.unwrap())
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profiles")]
pub async fn view_all_profiles(db: Data<MongoRepo>) -> HttpResponse {
    let user_detail = db.get_all_users().await;
    match user_detail {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/edit/{sub_id}")]
pub async fn edit_profile(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let sub_id = path.into_inner();
    if sub_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let(clone1, clone2) = (new_user.clone(), new_user.clone());
    let embeddings = update_embedding(new_user, clone2.vector_embeddings.unwrap()).await.expect("Error generating embeddings");
    let data = set_fields(clone1, embeddings, Some(sub_id.clone()));
    let update_result = db.update_user(&sub_id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&sub_id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/delete/{sub_id}")]
pub async fn delete_profile(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let sub_id = path.into_inner();
    if sub_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&sub_id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn update_embedding(mut user: Json<User>, old_embeddings: VectorEmbedding) -> Result<VectorEmbedding, reqwest::Error> {
    let mut embeddings = VectorEmbedding::default();
    if user.vector_embeddings.is_none() {
        user.vector_embeddings = Some(embeddings.clone());
    }

    embeddings = user.vector_embeddings.as_mut().unwrap().to_owned();

    if (embeddings.age.is_empty() || embeddings.age != old_embeddings.age) && user.age.is_some() {
        embeddings.age = generate_embedding(&user.age.unwrap().to_string()).await?;
    }
    if (embeddings.location.is_empty() || embeddings.location != old_embeddings.location) && user.location.is_some() {
        embeddings.location = generate_embedding(&user.location.clone().unwrap()).await?;
    }
    if (embeddings.employer.is_empty() || embeddings.employer != old_embeddings.employer) && user.employer.is_some() {
        embeddings.employer = generate_embedding(&user.employer.clone().unwrap()).await?;
    }
    if (embeddings.reason.is_empty() || embeddings.reason != old_embeddings.reason) && user.reason.is_some() {
        embeddings.reason = generate_embedding(&user.reason.clone().unwrap()).await?;
    }
    if (embeddings.project_interests.is_empty() || embeddings.project_interests != old_embeddings.project_interests) && user.project_interests.is_some() {
        embeddings.project_interests = generate_embedding(&user.project_interests.clone().unwrap()).await?;
    }
    if (embeddings.personality_interests.is_empty() || embeddings.personality_interests != old_embeddings.personality_interests) && user.personality_interests.is_some() {
        embeddings.personality_interests = generate_embedding(&user.personality_interests.clone().unwrap()).await?;
    }
    if (embeddings.skills.is_empty() || embeddings.skills != old_embeddings.skills) && user.skills.is_some() {
        embeddings.skills = generate_embedding(&user.skills.clone().unwrap()).await?;
    }
    if (embeddings.right_swipes.is_empty() || embeddings.right_swipes != old_embeddings.right_swipes) && user.right_swipes.is_some() {
        embeddings.right_swipes = generate_embedding(&user.right_swipes.clone().unwrap().join(",")).await?;
    }
    if (embeddings.left_swipes.is_empty() || embeddings.left_swipes != old_embeddings.left_swipes) && user.left_swipes.is_some() {
        embeddings.left_swipes = generate_embedding(&user.left_swipes.clone().unwrap().join(",")).await?;
    }
    if (embeddings.matches.is_empty() || embeddings.matches != old_embeddings.matches) && user.matches.is_some() {
        embeddings.matches = generate_embedding(&user.matches.clone().unwrap().join(",")).await?;
    }

    Ok(embeddings)
}

fn set_fields(new_user: User, embeddings: VectorEmbedding, sub_id: Option<String>) -> User {
    if sub_id.is_none() {
        panic!("No sub_id provided");
    }
    User {
        id: new_user.id.to_owned(),
        sub_id,
        first_name: new_user.first_name.to_owned(),
        last_name: new_user.last_name.to_owned(),
        email: new_user.email.to_owned(),
        github: new_user.github.to_owned(),
        website: new_user.website.to_owned(),
        age: new_user.age.to_owned(),
        location: new_user.location.to_owned(),
        employer: new_user.employer.to_owned(),
        reason: new_user.reason.to_owned(),
        project_interests: new_user.project_interests.to_owned(),
        personality_interests: new_user.personality_interests.to_owned(),
        skills: new_user.skills.to_owned(),
        right_swipes: new_user.right_swipes.to_owned(),
        left_swipes: new_user.left_swipes.to_owned(),
        matches: new_user.matches.to_owned(),
        public_fields: new_user.public_fields.to_owned(),
        vector_embeddings: Some(embeddings),
    }
}