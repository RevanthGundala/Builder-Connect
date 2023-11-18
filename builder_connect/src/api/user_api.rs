use crate::{models::user_model::{User, VectorEmbedding}, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post,
    get,
    put,
    delete,
    web::{Data, Json, Path},
    HttpResponse
};
use reqwest::Client;
use super::user_actions::generate_embedding;

#[get("/view/{sub_id}")]
pub async fn view_profile(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let sub_id = path.into_inner();
    if sub_id.is_empty() {
        return HttpResponse::BadRequest().body("invalid sub_id");
    }
    let user_detail = db.get_user(&sub_id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("No user found with specified ID"),
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
    let mut data;
    match update_embedding(new_user, clone2.vector_embeddings.unwrap()).await {
        Ok(embeddings) => {
            data = set_fields(clone1, Some(embeddings), Some(sub_id.clone()));
        }
        Err(_) => return HttpResponse::InternalServerError().body("Error generating embeddings"),
    }
    let update_result = db.update_user(&sub_id, data.clone()).await;
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

// TODO: FIgure out better way to generate embeddings outside of just proj intersts
async fn update_embedding(mut user: Json<User>, old_embeddings: Vec<f32>) -> Result<Vec<f32>, reqwest::Error> {
    let mut embeddings = user.vector_embeddings.as_mut().unwrap().to_owned();
    if (embeddings.is_empty() || embeddings != old_embeddings) && user.project_interests.is_some() {
        embeddings = generate_embedding(&user.project_interests.clone().unwrap()).await?;
    }
    Ok(embeddings)
}

fn set_fields(new_user: User, embeddings: Option<Vec<f32>>, sub_id: Option<String>) -> User {
    if sub_id.is_none() {
        panic!("No sub_id provided");
    }
    User {
        id: new_user.id.to_owned(),
        sub_id,
        image_url: new_user.image_url.to_owned(),
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        discord: new_user.discord.to_owned(),
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
        vector_embeddings: embeddings,
    }
}