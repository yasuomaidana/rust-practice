mod files_url;
mod ignored_segments;
mod simple_url;

#[macro_use]
extern crate rocket;

use crate::files_url::image;
use crate::ignored_segments::*;
use crate::simple_url::*;
use rocket::fs::FileServer;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[launch]
fn rocket() -> _ {
    
    /* 
    Logging using the tracing crate
    Initialize advanced logging with tracing
    */

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global default");

    // Default logging with tracing
    // tracing_subscriber::fmt::init();

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
