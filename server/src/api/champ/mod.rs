mod data;

pub use data::Champ;

use crate::error::ResourceNotFoundError;
use crate::state::AppState;
use rocket::State;
use rocket::{serde::json::Json, Route};

#[get("/champ/<id>")]
pub async fn id(state: &State<AppState>, id: i64) -> Json<Result<Champ, ResourceNotFoundError>> {
    let champ = Champ::get_by_id(&state.pool, id).await;

    Json(champ.ok_or(ResourceNotFoundError {}))
}

pub fn routes() -> Vec<Route> {
    routes![name, id]
}

#[get("/champ/<name>", rank = 2)]
pub async fn name(
    state: &State<AppState>,
    name: String,
) -> Json<Result<Champ, ResourceNotFoundError>> {
    let champ = Champ::get_by_name(&state.pool, name).await;

    Json(champ.ok_or(ResourceNotFoundError {}))
}
