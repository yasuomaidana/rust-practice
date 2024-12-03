mod endpoint_examples;
mod states_example;
mod utils;
mod templates_example;

use crate::endpoint_examples::scopes_example::files;
use crate::endpoint_examples::simple_endpoints::*;
use crate::states_example::states_endpoints::state_endpoints;
use actix_web::{App, HttpServer};
use crate::templates_example::templates_endpoint::templates_endpoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(state_endpoints())
            .service(files())
            .service(templates_endpoint())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
