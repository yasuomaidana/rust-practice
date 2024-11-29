use std::path::PathBuf;

#[get("/ignored_segments/<_>/<ending>", rank = 2)]
pub fn ignored_ending(ending: String) -> String {
    format!("Ignored segments: {}", ending)
}

#[get("/ignored_segments/files/<file..>", rank = 1)]
pub fn ignored_json_files(file:PathBuf) -> String {
    if let Some(ext) = file.extension() {
        if ext == "json" {
            return format!("Ignored json files: {:?}", file);
        }
    }
    "Ignored but not json files".to_string()
}

#[get("/ignored_segments2/<_..>")]
pub fn ignored() -> String {
    "Ignored all segments here".to_string()
}