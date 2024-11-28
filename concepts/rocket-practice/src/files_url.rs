use rocket::tokio;
use std::path::PathBuf;

#[get("/image/<filename..>")]
pub(crate) async fn image(filename: PathBuf) -> Option<tokio::fs::File> {
    let path = PathBuf::from("static").join(&filename);

    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "png" || ext == "jpg" {
                return Some(tokio::fs::File::open(path).await.unwrap());
            }
        }
    }
    None
}
