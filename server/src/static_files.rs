use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

async fn get_static_resource(path: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("public/{}", path)))
        .await
        .ok()
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    get_static_resource("index.html").await
}

#[get("/favicon.ico")]
pub async fn favicon() -> Option<NamedFile> {
    get_static_resource("favicon.ico").await
}

#[get("/assets/<file..>")]
pub async fn assets_file(file: PathBuf) -> Option<NamedFile> {
    if let Some(file) = file.to_str() {
        get_static_resource(&format!("assets/{}", file)).await
    } else {
        None
    }
}
