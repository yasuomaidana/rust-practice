use actix_web::{get, web, Responder};
use handlebars::{DirectorySourceOptions, Handlebars};
use serde::{Deserialize, Serialize};
use crate::*;

#[derive(Serialize, Deserialize)]
struct UserData {
    user: String,
    data: String,
}

#[get("/user")]
pub async fn get_user(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = UserData {
        user: "John Doe".to_string(),
        data: "Some data".to_string(),
    };
    // let data = json!(data);
    let body = hb.render("user", &data).unwrap();
    web::Html::new(body)
}

pub fn templates_endpoint() -> actix_web::Scope {
    let scope = web::scope("/templates");

    let mut handlebars = Handlebars::new();
    let mut options = DirectorySourceOptions::default();
    options.tpl_extension = ".html".to_string();

    handlebars
        .register_templates_directory("./templates", options)
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    add_into_scope_with_mutable_struct_data!(scope, handlebars_ref, get_user)
}
