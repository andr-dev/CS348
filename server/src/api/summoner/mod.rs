use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::ServiceError;
use crate::state::AppState;
use rocket::State;
use rocket::{serde::json::Json, Route};

use super::matches::DbMatch;

mod db;
pub use db::DbSummoner;

#[get("/summoner/<name>")]
pub async fn name(state: &State<AppState>, name: String) -> Json<Result<DbSummoner, ServiceError>> {
    let mut update = false;

    if let Ok(summoner) = DbSummoner::get_by_summoner_name(&state.pool, &name).await {
        if ((summoner.lastupdate.timestamp() + 60 * 5) as u128)
            > SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards!")
                .as_millis()
        {
            return Json(Ok(summoner));
        }

        update = true;
    }

    println!("HERE: {:?}", update);

    if let Ok(summoner) = state.rito_client.get_summoner_by_summoner_name(&name).await {
        println!("SUMV4: {:?}", summoner);

        let dbsum = DbSummoner::from_summoner_v4(summoner);

        println!("DBSUM: {:?}", dbsum);

        if !(if update {
            DbSummoner::update_summoner(&state.pool, &dbsum)
                .await
                .is_ok()
        } else {
            DbSummoner::insert_summoner(&state.pool, &dbsum)
                .await
                .is_ok()
        }) {
            return Json(Err(ServiceError {
                error: format!(
                    "failed to {} summoner",
                    if update { "update" } else { "insert" }
                )
                .into(),
            }));
        }

        Json(update_summoner_games(&state, &dbsum).await.map(|_| dbsum))
    } else {
        Json(Err(ServiceError {
            error: "failed to fetch summoner".into(),
        }))
    }
}

pub async fn update_summoner_games(
    state: &State<AppState>,
    summoner: &DbSummoner,
) -> Result<(), ServiceError> {
    let match_ids = state
        .rito_client
        .get_match_ids_by_puuid(&summoner.puuid)
        .await
        .map_err(|e| ServiceError { error: Box::new(e) })?;

    for match_id in match_ids {
        // Only fetch if match does not yet exist in database or TODO: If end_timestamp is null and match could now be over
        if DbMatch::find_by_match_id(&state.pool, &match_id)
            .await
            .is_none()
        {
            println!("MATCH ID {:?} does not exist", match_id);

            match state.rito_client.get_match_by_match_id(&match_id).await {
                Ok(rito_match) => {
                    let db_match = DbMatch::from_match_v5(&rito_match);

                    // First, fetch all participants to satisfy participant SQL references
                    for puuid in get_participants(&db_match) {
                        if DbSummoner::get_by_summoner_puuid(&state.pool, &puuid.to_string())
                            .await
                            .is_err()
                        {
                            let summoner = state
                                .rito_client
                                .get_summoner_by_summoner_puuid(puuid)
                                .await
                                .map_err(|e| ServiceError { error: Box::new(e) })?;

                            let db_summoner = DbSummoner::from_summoner_v4(summoner);

                            DbSummoner::insert_summoner(&state.pool, &db_summoner)
                                .await
                                .map_err(|e| ServiceError { error: Box::new(e) })?;
                        }
                    }

                    DbMatch::insert_match(&state.pool, summoner, rito_match, db_match).await?;

                    println!("DONE ADDING");
                }
                Err(e) => match e {
                    rito::apis::Error::Reqwest(e) => {
                        println!("REQWEST ERROR: {:?}", e);
                    }
                    rito::apis::Error::Serde(e) => {
                        println!("SERDE ERROR: {:?}", e);
                    }
                    rito::apis::Error::Io(e) => {
                        println!("IO ERROR: {:?}", e);
                    }
                    rito::apis::Error::ResponseError(e) => {
                        println!("RESPONSE ERROR: {:?}", e);
                    } // _ => {
                      //     return Err(ServiceError { error: Box::new(e) });
                      // }
                },
            }
        }
    }

    println!("OK!");

    Ok(())
}

fn get_participants(db_match: &DbMatch) -> [&str; 10] {
    [
        &db_match.participant0,
        &db_match.participant1,
        &db_match.participant2,
        &db_match.participant3,
        &db_match.participant4,
        &db_match.participant5,
        &db_match.participant6,
        &db_match.participant7,
        &db_match.participant8,
        &db_match.participant9,
    ]
}

#[get("/summoner/<puuid>/kda")]
pub async fn kda(
    state: &State<AppState>,
    puuid: String,
) -> Json<Result<(f64, f64, f64), ServiceError>> {
    Json(
        DbMatch::get_kda_by_puuid(&state.pool, &puuid)
            .await
            .map_err(|_| ServiceError {
                error: format!("failed to fetch summoner with puuid {}", puuid).into(),
            }),
    )
}

pub fn routes() -> Vec<Route> {
    routes![name, kda]
}
