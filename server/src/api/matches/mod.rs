mod db;

pub use db::DbMatch;

use crate::error::ServiceError;
use crate::state::AppState;
use rocket::State;
use rocket::{serde::json::Json, Route};

#[get("/matches/<gameid>")]
pub async fn gameid(
    state: &State<AppState>,
    gameid: String,
) -> Json<Result<DbMatch, ServiceError>> {
    let match_obj = DbMatch::find_by_match_id(&state.pool, &gameid).await;

    Json(match_obj.ok_or(ServiceError {
        error: format!("No match found with gameid {}", gameid).into(),
    }))
}

pub fn routes() -> Vec<Route> {
    routes![gameid]
}
