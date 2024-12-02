use actix_web::{get, web, HttpResponse, Responder, Scope};
use log::info;
use crate::add_path_with_data;

pub struct StringData {
    pub message: String,
}

pub async fn manual_message(message: web::Data<StringData>) -> impl Responder {
    let message = message.message.to_string();
    info!("Request received: {}", message);
    HttpResponse::Ok().body(message)
}

pub fn manual_message_factory(path: &str, message: String) -> Scope {
    web::scope(path)
        .app_data(web::Data::new(StringData { message }))
        .route("", web::get().to(manual_message))
}

#[get("/buu")]
pub async fn buu(message: web::Data<StringData>) -> impl Responder {
    let message = message.message.to_string();
    info!("Request received: {}", message);
    HttpResponse::Ok().body(message)
}

pub fn state_endpoints() -> Scope {
    web::scope("")
        .service(manual_message_factory("/hey", "Hey there!".to_string()))
        .service(manual_message_factory("/bye", "Bye bye!".to_string()))
        .service(add_path_with_data!(buu, StringData { message: "Hello world! buu".to_string() }))
}
