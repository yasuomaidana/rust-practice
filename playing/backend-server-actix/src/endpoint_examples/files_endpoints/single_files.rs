use actix_files::NamedFile;
use actix_web::get;
use std::path::PathBuf;

#[get("/")]
pub async fn app_index() -> actix_web::Result<NamedFile> {
    let path = PathBuf::from("index.html");
    Ok(NamedFile::open(path)?)
}

pub async fn manual_index() -> actix_web::Result<NamedFile> {
    let path = PathBuf::from("index.html");
    Ok(NamedFile::open(path)?)
}
