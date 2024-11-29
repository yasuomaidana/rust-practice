use rocket::tokio;
use std::path::PathBuf;
// use tracing::{info, warn, debug};
use log::{info, warn, debug};

#[get("/image/<filename..>")]
pub(crate) async fn image(filename: PathBuf) -> Option<tokio::fs::File> {
    let path = PathBuf::from("static").join(&filename);

    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "png" || ext == "jpg" {
                info!("Serving image: {:?}", path);
                return Some(tokio::fs::File::open(path).await.unwrap());
            }
            else{
                debug!("Not an image: {:?}", path);
            }
        }
    }
    else{
        warn!("File not found: {:?}", path);
    }
    None
}
