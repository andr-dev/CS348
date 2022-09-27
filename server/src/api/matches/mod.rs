mod data;

pub use data::Match;

use crate::error::ResourceNotFoundError;
use rocket::State;
use rocket::{serde::json::Json, Route};
use sqlx::{Pool, Sqlite};

#[get("/matches/<gameid>")]
pub async fn gameid(
    pool: &State<Pool<Sqlite>>,
    gameid: i64,
) -> Json<Result<Match, ResourceNotFoundError>> {
    let match_obj = Match::get_by_gameid(pool, gameid).await;

    Json(match_obj.ok_or(ResourceNotFoundError {}))
}

pub fn routes() -> Vec<Route> {
    routes![gameid]
}
