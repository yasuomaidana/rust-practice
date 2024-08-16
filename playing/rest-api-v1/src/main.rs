mod api;
mod models;
mod cache;

use actix_web::{ App, HttpServer};
use crate::api::get_pokemon;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pokemon)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}