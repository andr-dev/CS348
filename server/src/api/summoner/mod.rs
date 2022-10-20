use crate::error::ResourceNotFoundError;
use crate::state::AppState;
use rito::models::SummonerV4PeriodSummonerDto;
use rocket::State;
use rocket::{serde::json::Json, Route};

#[get("/summoner/<name>")]
pub async fn name(
    state: &State<AppState>,
    name: String,
) -> Json<Result<SummonerV4PeriodSummonerDto, ResourceNotFoundError>> {
    let summoner = state.rito_client.get_by_summoner_name(name).await;

    Json(summoner.ok().ok_or(ResourceNotFoundError {}))
}

pub fn routes() -> Vec<Route> {
    routes![name]
}
