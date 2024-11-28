#[macro_use] extern crate rocket;

use std::time::Duration;
use rocket::tokio::time::sleep;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/", routes![index, delay, delay_post],
    )
}
