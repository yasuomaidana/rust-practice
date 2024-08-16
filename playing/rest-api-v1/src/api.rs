use actix_web::{get, web, HttpResponse, Responder};

use super::cache::get_cached_pokemon;

#[get("/pokemon/{pokemon_name}")]
async fn get_pokemon(pokemon_name: web::Path<String>) -> impl Responder {
    match get_cached_pokemon(&pokemon_name).await {
        Ok(pokemon) => HttpResponse::Ok().json(pokemon),
        Err(err) => HttpResponse::InternalServerError().json(err.error),
    }
}