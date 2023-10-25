use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post,
    get,
    put,
    delete,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/profile")]
pub async fn create_profile(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
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
        incoming_right_swipes: new_user.incoming_right_swipes.to_owned(),
        incoming_left_swipes: new_user.incoming_left_swipes.to_owned(),
        matches: new_user.matches.to_owned(),
        public_fields: new_user.public_fields.to_owned(),
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profile/{id}")]
pub async fn view_profile(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/profile/{id}")]
pub async fn edit_profile(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
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
        incoming_right_swipes: new_user.incoming_right_swipes.to_owned(),
        incoming_left_swipes: new_user.incoming_left_swipes.to_owned(),
        matches: new_user.matches.to_owned(),
        public_fields: new_user.public_fields.to_owned(),
    };
    let update_result = db.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
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
#[delete("/profile/{id}")]
pub async fn delete_profile(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;
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