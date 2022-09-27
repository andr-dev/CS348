mod data;

pub use data::Champ;

use crate::error::ResourceNotFoundError;
use rocket::State;
use rocket::{serde::json::Json, Route};
use sqlx::{Pool, Sqlite};

#[get("/champ/<id>")]
pub async fn id(pool: &State<Pool<Sqlite>>, id: i64) -> Json<Result<Champ, ResourceNotFoundError>> {
    let champ = Champ::get_by_id(pool, id).await;

    Json(champ.ok_or(ResourceNotFoundError {}))
}

pub fn routes() -> Vec<Route> {
    routes![name, id]
}

#[get("/champ/<name>", rank = 2)]
pub async fn name(
    pool: &State<Pool<Sqlite>>,
    name: String,
) -> Json<Result<Champ, ResourceNotFoundError>> {
    let champ = Champ::get_by_name(pool, name).await;

    Json(champ.ok_or(ResourceNotFoundError {}))
}
