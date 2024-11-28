mod simple_url;
mod files_url;

#[macro_use]
extern crate rocket;

use crate::files_url::image;
use crate::simple_url::*;
use rocket::fs::FileServer;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, delay, delay_post, image])
        .mount("/public", FileServer::from("static"))
}
