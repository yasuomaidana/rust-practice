use crate::add_into_scope;
use crate::endpoint_examples::files_endpoints::single_files::{app_index_file, manual_index_file};
use actix_web::{web, Scope};

pub fn files() -> Scope {
    let scope = web::scope("/files");
    let scope = add_into_scope!(scope, app_index_file);
    let scope = add_into_scope!(scope, web::get, "", manual_index_file);
    scope
        /*
        /// Serve static files from the /public directory
        /// To access this file, go to http://localhost:8080/public/Filename
        /// it will serve the file from the static directory
         */
        .service(actix_files::Files::new("/public", "./static").show_files_listing())
}
