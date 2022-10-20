mod data;

pub use data::Match;

use crate::error::ResourceNotFoundError;
use crate::state::AppState;
use rocket::State;
use rocket::{serde::json::Json, Route};

#[get("/matches/<gameid>")]
pub async fn gameid(
    state: &State<AppState>,
    gameid: i64,
) -> Json<Result<Match, ResourceNotFoundError>> {
    let match_obj = Match::get_by_gameid(&state.pool, gameid).await;

    Json(match_obj.ok_or(ResourceNotFoundError {}))
}

pub fn routes() -> Vec<Route> {
    routes![gameid]
}
