use std::future::Future;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(message: String) -> impl Responder {
    HttpResponse::Ok().body(message)
}

fn func_wrapper(message: String) -> impl Future<Output = impl Responder> {
    manual_hello(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(|| func_wrapper("Hey there!".to_string())))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
