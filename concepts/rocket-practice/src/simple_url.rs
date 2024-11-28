use rocket::tokio::time::sleep;
use std::time::Duration;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[post("/delay/<seconds>")]
pub async fn delay_post(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds post", seconds)
}
