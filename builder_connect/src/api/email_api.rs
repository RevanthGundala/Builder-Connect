use actix_web::{get, post, delete, web::{self, Path}, HttpResponse, Responder};
use crate::repository::mongodb_repo::MongoRepo;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

#[post("/mailing_list/{email}")]
pub async fn add_to_mailing_list(
    email: Path<String>,
    db: web::Data<MongoRepo>,
) -> impl Responder {
    let res = db.add_to_mailing_list(email.into_inner()).await;
    println!("{:?}", res);
    match res {
        Ok(_) => HttpResponse::Ok().body("Added to mailing list"),
        Err(_) => HttpResponse::InternalServerError().body("Error adding to mailing list"),
    }
}

#[delete("/mailing_list/{email}")]
pub async fn delete_from_mailing_list(
    email: Path<String>,
    db: web::Data<MongoRepo>,
) -> impl Responder {
    let res = db.delete_from_mailing_list(email.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("Deleted from mailing list"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting from mailing list"),
    }
}

#[get("/test_email")]
pub async fn send_email() -> impl Responder { 
    let email_username = match env::var("BUILDER_CONNECT_EMAIL") {
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
    let email_password = match env::var("BUILDER_CONNECT_EMAIL_PASSWORD") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading email password"),
    };

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