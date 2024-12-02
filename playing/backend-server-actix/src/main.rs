mod endpoint_examples;
mod states_example;
mod utils;

use crate::endpoint_examples::scopes_example::files;
use crate::endpoint_examples::simple_endpoints::*;
use crate::states_example::states_endpoints::state_endpoints;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(state_endpoints())
            .service(files())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
