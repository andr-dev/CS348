extern crate proc_macro;

#[macro_use]
extern crate rocket;

use cors::CORS;
use rocket::{Build, Rocket};
use state::AppState;

mod api;
mod cors;
mod error;
mod queries;
mod rito;
mod state;
mod static_files;

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                static_files::index,
                static_files::favicon,
                static_files::assets_file,
            ],
        )
        .mount("/api", api::routes())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::new().await?;

    let _ = rocket()
        .manage::<AppState>(app_state)
        .attach(CORS)
        .launch()
        .await?;

    Ok(())
}
