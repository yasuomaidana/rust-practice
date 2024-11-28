#[macro_use]
extern crate rocket;

use rocket::tokio;
use rocket::tokio::time::sleep;
use std::path::PathBuf;
use std::time::Duration;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[post("/delay/<seconds>")]
async fn delay_post(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds post", seconds)
}

#[get("/image/<filename..>")]
async fn image(filename: PathBuf) -> Option<tokio::fs::File> {
    let path = PathBuf::from("static").join(filename);
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "png" || ext == "jpg" {
                return Some(tokio::fs::File::open(path).await.unwrap());
            }
        }
    }
    None
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",
               routes![
                   index, 
                   delay, 
                   delay_post,
                   image
               ])
}
