use crate::{add_into_scope, add_into_scope_with_inmutable_struct_data,add_struct_data_into_scope};
use actix_web::{get, put, web, HttpResponse, Responder, Scope};
use log::info;

pub struct StringData {
    pub message: String,
}

pub async fn manual_message(message: web::Data<StringData>) -> impl Responder {
    let message = message.message.to_string();
    info!("Request received: {}", message);
    HttpResponse::Ok().body(message)
}

pub fn manual_message_factory(path: &str, message: String) -> Scope {
    add_into_scope!(
        web::scope(path).app_data(web::Data::new(StringData { message })),
        web::get,
        "",
        manual_message
    )
}

#[get("/buu")]
pub async fn buu(message: web::Data<StringData>) -> impl Responder {
    let message = message.message.to_string();
    info!("Request received: {}", message);
    HttpResponse::Ok().body(message)
}

#[get("/bye-bye")]
pub async fn second_bye(message: web::Data<StringData>) -> impl Responder {
    let message = message.message.to_string();
    info!("Request get received: {}", message);
    HttpResponse::Ok().body(message)
}

#[put("/bye-bye")]
pub async fn second_bye_put(message: web::Data<StringData>) -> impl Responder {
    let message = message.message.to_string();
    info!("Request put received: {}", message);
    HttpResponse::Ok().body(format!("{} (PUT)", message))
}

pub fn state_endpoints() -> Scope {
    let scope = web::scope("/states")
        .service(manual_message_factory("/hey", "Hey there!".to_string()))
        .service(manual_message_factory("/bye", "Bye bye!".to_string()));
    let message = StringData {
        message: "Hello world!".to_string(),
    };
    let scope = add_into_scope_with_inmutable_struct_data!(scope, message, buu);

    let message = StringData {
        message: "Bye endpoint message".to_string(),
    };
    
    add_into_scope_with_inmutable_struct_data!(scope, message, (second_bye,second_bye_put))
}
