mod files_url;
mod ignored_segments;
mod simple_url;

#[macro_use]
extern crate rocket;

use crate::files_url::image;
use crate::ignored_segments::*;
use crate::simple_url::*;
use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                delay,
                delay_post,
                image,
                ignored_ending,
                ignored,
                ignored_json_files
            ],
        )
        .mount("/public", FileServer::from("static"))
}
