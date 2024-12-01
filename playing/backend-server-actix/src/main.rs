mod files_scope;

use crate::files_scope::files;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use std::future::Future;
use std::path::PathBuf;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(message: String) -> impl Responder {
    info!("Request received: {}", &message);
    HttpResponse::Ok().body(message)
}

fn func_wrapper(message: String) -> impl Future<Output = impl Responder> {
    manual_hello(message)
}

#[get("/")]
async fn app_index() -> actix_web::Result<NamedFile> {
    let path = PathBuf::from("index.html");
    Ok(NamedFile::open(path)?)
}

async fn manual_index() -> actix_web::Result<NamedFile> {
    let path = PathBuf::from("index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route(
                "/hey",
                web::get().to(|| func_wrapper("Hey there!".to_string())),
            )
            .service(files())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
