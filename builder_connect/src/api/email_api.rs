use actix_web::{get, post, delete, web::{self, Path}, HttpResponse, Responder};
use crate::{repository::mongodb_repo::MongoRepo, api::user_actions::send_email};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

#[post("/mailing_list/{email}")]
pub async fn add_to_mailing_list(
    email: Path<String>,
    db: web::Data<MongoRepo>,
) -> impl Responder {
    let email = email.into_inner();
    let res = db.add_to_mailing_list(email.clone()).await;
    println!("{:?}", res);
    match res {
        Ok(_) => {
            let _ = send_email(email, String::from("Welcome to the BuildWork"), String::from("You have signed up for notifications successfully!")).await;
            HttpResponse::Ok().body("Added to mailing list")
        }
        Err(_) => HttpResponse::InternalServerError().body("Error adding to mailing list"),
    }
}

#[delete("/mailing_list/{email}")]
pub async fn delete_from_mailing_list(
    email: Path<String>,
    db: web::Data<MongoRepo>,
) -> impl Responder {
    let email = email.into_inner();
    let res = db.delete_from_mailing_list(email.clone()).await;
    match res {
        Ok(_) => {
            let _ = send_email(email, String::from("Unsubscribed from BuildWork"), String::from("You will no longer receive emails from the BuildWork")).await;
            HttpResponse::Ok().body("Deleted from mailing list")
        }
        Err(_) => HttpResponse::InternalServerError().body("Error deleting from mailing list"),
    }
}

#[get("/test_email")]
pub async fn test_send_email() -> impl Responder { 
    let email_username = match env::var("BUILDWORK_EMAIL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading email username"),
    };
    let email = Message::builder()
        .from(format!("<{email_username}>").parse().unwrap())
        .to(format!("<{email_username}>").parse().unwrap())
        .subject("Test")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Test From BuilderConnect"))
        .unwrap();
    let email_password = match env::var("BUILDWORK_EMAIL_PASSWORD") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading email password"),
    };
    println!("Password: {email_password}");
    let creds = Credentials::new(email_username, email_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => HttpResponse::Ok().body("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}