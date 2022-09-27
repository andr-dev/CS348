#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

mod api;
mod error;
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
    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let _ = rocket().manage::<Pool<Sqlite>>(pool).launch().await?;

    Ok(())
}
