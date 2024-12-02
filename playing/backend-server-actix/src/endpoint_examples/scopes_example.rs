use actix_web::{web, Scope};
use crate::endpoint_examples::files_endpoints::single_files::app_index;

pub fn files() -> Scope {
    web::scope("/files")
        .service(app_index)
        .route("", web::get().to(crate::endpoint_examples::files_endpoints::single_files::manual_index))
        /*
        /// Serve static files from the /public directory
        /// To access this file, go to http://localhost:8080/public/Filename
        /// it will serve the file from the static directory
         */
        .service(actix_files::Files::new("/public", "./static").show_files_listing())
}