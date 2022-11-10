mod data;

pub use data::DbChampion;

use crate::error::ServiceError;
use crate::state::AppState;
use rocket::State;
use rocket::{serde::json::Json, Route};

#[get("/champion/id/<id>")]
pub async fn id(state: &State<AppState>, id: i64) -> Json<Result<DbChampion, ServiceError>> {
    let champ = DbChampion::get_by_id(&state.pool, id).await;

    Json(champ.ok_or(ServiceError {
        error: format!("No champ with id {}", id).into(),
    }))
}

#[get("/champion/name/<name>")]
pub async fn name(state: &State<AppState>, name: String) -> Json<Result<DbChampion, ServiceError>> {
    let champ = DbChampion::get_by_name(&state.pool, &name).await;

    Json(champ.ok_or(ServiceError {
        error: format!("No champ with name {}", name).into(),
    }))
}

#[get("/champion/winrate?<min>&<max>")]
pub async fn winrate(state: &State<AppState>, min: u32, max: u32) -> Json<Result<Vec<(i64, f64)>, ServiceError>> {
    let winrate = DbChampion::get_win_rate(&state.pool, min, max).await;

    Json(winrate)
}

pub fn routes() -> Vec<Route> {
    routes![name, id, winrate]
}
