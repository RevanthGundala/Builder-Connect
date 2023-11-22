use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post,
    get,
    put,
    delete,
    web::{Data, Json, Path},
    HttpResponse
};
use super::user_actions::generate_embedding;
use crate::models::user_model::UserView;
use mongodb::bson::oid::ObjectId;

// test
#[post("/create")]
pub async fn create_many_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = vec![
        User {
            id: Some(ObjectId::new()),
            sub_id: Some(String::from("123456")),
            image_url: String::from("https://example.com/user1.jpg"),
            username: String::from("user1"),
            email: String::from("user1@example.com"),
            discord: String::from("user1#1234"),
            github: Some(String::from("user1github")),
            website: Some(String::from("https://user1.com")),
            age: Some(String::from("25")),
            location: Some(String::from("City1, Country1")),
            employer: Some(String::from("Company1")),
            reason: Some(String::from("Passionate about technology")),
            project_interests: Some(String::from("Blockchain, AI")),
            personality_interests: Some(String::from("Introverted, Analytical")),
            skills: Some(String::from("Python, JavaScript, Solidity")),
            right_swipes: Some(vec![]),
            left_swipes: Some(vec![]),
            matches: Some(vec![]),
            public_fields: Some(UserView::default()),
            vector_embeddings: Some(vec![]),
        },
        User {
            id: Some(ObjectId::new()),
            sub_id: Some(String::from("789012")),
            image_url: String::from("https://example.com/user2.jpg"),
            username: String::from("user2"),
            email: String::from("user2@example.com"),
            discord: String::from("user2#5678"),
            github: Some(String::from("user2github")),
            website: Some(String::from("https://user2.com")),
            age: Some(String::from("30")),
            location: Some(String::from("City2, Country2")),
            employer: Some(String::from("Company2")),
            reason: Some(String::from("Excited to collaborate on projects")),
            project_interests: Some(String::from("IoT, Data Science")),
            personality_interests: Some(String::from("Friendly, Creative")),
            skills: Some(String::from("Java, C++, Python")),
            right_swipes: Some(vec![]),
            left_swipes: Some(vec![]),
            matches: Some(vec![]),
            public_fields: Some(UserView::default()),
            vector_embeddings: Some(vec![]),
        },
        User {
            id: Some(ObjectId::new()),
            sub_id: Some(String::from("345678")),
            image_url: String::from("https://example.com/user3.jpg"),
            username: String::from("user3"),
            email: String::from("user3@example.com"),
            discord: String::from("user3#9012"),
            github: Some(String::from("user3github")),
            website: Some(String::from("https://user3.com")),
            age: Some(String::from("22")),
            location: Some(String::from("City3, Country3")),
            employer: Some(String::from("University3")),
            reason: Some(String::from("Keen on learning new technologies")),
            project_interests: Some(String::from("Mobile App Development")),
            personality_interests: Some(String::from("Outgoing, Adventurous")),
            skills: Some(String::from("Swift, Kotlin, React Native")),
            right_swipes: Some(vec![]),
            left_swipes: Some(vec![]),
            matches: Some(vec![]),
            public_fields: Some(UserView::default()),
            vector_embeddings: Some(vec![]),
        },
        User {
            id: Some(ObjectId::new()),
            sub_id: Some(String::from("567890")),
            image_url: String::from("https://example.com/user4.jpg"),
            username: String::from("user4"),
            email: String::from("user4@example.com"),
            discord: String::from("user4#3456"),
            github: Some(String::from("user4github")),
            website: Some(String::from("https://user4.com")),
            age: Some(String::from("28")),
            location: Some(String::from("City4, Country4")),
            employer: Some(String::from("Startup4")),
            reason: Some(String::from("Passionate about entrepreneurship")),
            project_interests: Some(String::from("Artificial Intelligence")),
            personality_interests: Some(String::from("Optimistic, Ambitious")),
            skills: Some(String::from("JavaScript, Node.js, TensorFlow")),
            right_swipes: Some(vec![]),
            left_swipes: Some(vec![]),
            matches: Some(vec![]),
            public_fields: Some(UserView::default()),
            vector_embeddings: Some(vec![]),
        },
        User {
            id: Some(ObjectId::new()),
            sub_id: Some(String::from("901234")),
            image_url: String::from("https://example.com/user5.jpg"),
            username: String::from("user5"),
            email: String::from("user5@example.com"),
            discord: String::from("user5#6789"),
            github: Some(String::from("user5github")),
            website: Some(String::from("https://user5.com")),
            age: Some(String::from("35")),
            location: Some(String::from("City5, Country5")),
            employer: Some(String::from("Company5")),
            reason: Some(String::from("Enthusiastic about open source projects")),
            project_interests: Some(String::from("DevOps, Cloud Computing")),
            personality_interests: Some(String::from("Analytical, Detail-oriented")),
            skills: Some(String::from("Docker, Kubernetes, AWS")),
            right_swipes: Some(vec![]),
            left_swipes: Some(vec![]),
            matches: Some(vec![]),
            public_fields: Some(UserView::default()),
            vector_embeddings: Some(vec![]),
        },
    ];
    let mut res;
    for user in users {
        res = db
            .col
            .insert_one(user.clone(), None)
            .await
            .ok()
            .expect("Error creating user");
        let client = reqwest::Client::new();
        client  
            .put("http://localhost:8080/edit/".to_string() + user.sub_id.as_ref().unwrap())
            .json(&user)
            .send()
            .await
            .expect("Error updating user");
    }
    return HttpResponse::Ok().json("Users successfully created!");
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