#[macro_export]
macro_rules! add_path_with_data {
    ($path:expr, $func:ident, $data:expr) => {
        // use actix_web::{web};
        web::scope($path)
            .app_data(web::Data::new($data))
            .service($func)
    };
    ($func:ident, $data:expr) => {
        // use actix_web::{web};
        web::scope("")
            .app_data(web::Data::new($data))
            .service($func)
    };
    // ($path:expr,$data:expr, $func:ident) => {
    //     // use actix_web::{web};
    //     
    // };
}
