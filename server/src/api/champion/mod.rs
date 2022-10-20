mod data;

pub use data::Champion;

use crate::error::ServiceError;
use crate::state::AppState;
use rocket::State;
use rocket::{serde::json::Json, Route};

#[get("/champion/<id>")]
pub async fn id(state: &State<AppState>, id: i64) -> Json<Result<Champion, ServiceError>> {
    let champ = Champion::get_by_id(&state.pool, id).await;

    Json(champ.ok_or(ServiceError {
        error: format!("No champ with id {}", id).into(),
    }))
}

#[get("/champion/<name>", rank = 2)]
pub async fn name(state: &State<AppState>, name: String) -> Json<Result<Champion, ServiceError>> {
    let champ = Champion::get_by_name(&state.pool, &name).await;

    Json(champ.ok_or(ServiceError {
        error: format!("No champ with name {}", name).into(),
    }))
}

pub fn routes() -> Vec<Route> {
    routes![name, id]
}
