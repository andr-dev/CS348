use rito::models::MatchV5PeriodMatchDto;
use rocket::serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

use crate::{api::summoner::DbSummoner, error::ServiceError};

#[derive(FromRow, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DbMatch {
    pub matchid: String,
    pub data_version: String,
    pub participant0: String,
    pub participant1: String,
    pub participant2: String,
    pub participant3: String,
    pub participant4: String,
    pub participant5: String,
    pub participant6: String,
    pub participant7: String,
    pub participant8: String,
    pub participant9: String,
    pub gameid: i64,
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_start_timestamp: i64,
    pub game_end_timestamp: Option<i64>,
}

impl DbMatch {
    pub async fn find_by_match_id(pool: &Pool<Sqlite>, match_id: &str) -> Option<DbMatch> {
        let r = sqlx::query_as::<_, DbMatch>("SELECT * FROM matches WHERE matchid = ?")
            .bind(match_id)
            .fetch_one(pool)
            .await;

        println!("R: {:?}", r);

        r.ok()
    }

    pub async fn insert_match(
        pool: &Pool<Sqlite>,
        summoner: &DbSummoner,
        rito_match: MatchV5PeriodMatchDto,
        db_match: Self,
    ) -> Result<(), ServiceError> {
        sqlx::query(
            "INSERT INTO matches VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(db_match.matchid.clone())
        .bind(db_match.data_version)
        .bind(db_match.participant0)
        .bind(db_match.participant1)
        .bind(db_match.participant2)
        .bind(db_match.participant3)
        .bind(db_match.participant4)
        .bind(db_match.participant5)
        .bind(db_match.participant6)
        .bind(db_match.participant7)
        .bind(db_match.participant8)
        .bind(db_match.participant9)
        .bind(db_match.gameid)
        .bind(db_match.game_creation)
        .bind(db_match.game_duration)
        .bind(db_match.game_start_timestamp)
        .bind(db_match.game_end_timestamp)
        .execute(pool)
        .await
        .map_err(|e| ServiceError { error: Box::new(e) })?;

        sqlx::query("INSERT INTO summoner_matches VALUES (?, ?)")
            .bind(summoner.puuid.clone())
            .bind(db_match.matchid)
            .execute(pool)
            .await
            .map_err(|e| ServiceError { error: Box::new(e) })
            .map(|_| ())
    }

    pub fn from_match_v5(match_v5: &MatchV5PeriodMatchDto) -> Self {
        Self {
            matchid: match_v5.metadata.match_id.clone(),
            data_version: match_v5.metadata.data_version.clone(),
            participant0: match_v5.info.participants.get(0).unwrap().puuid.clone(),
            participant1: match_v5.info.participants.get(1).unwrap().puuid.clone(),
            participant2: match_v5.info.participants.get(2).unwrap().puuid.clone(),
            participant3: match_v5.info.participants.get(3).unwrap().puuid.clone(),
            participant4: match_v5.info.participants.get(4).unwrap().puuid.clone(),
            participant5: match_v5.info.participants.get(5).unwrap().puuid.clone(),
            participant6: match_v5.info.participants.get(6).unwrap().puuid.clone(),
            participant7: match_v5.info.participants.get(7).unwrap().puuid.clone(),
            participant8: match_v5.info.participants.get(8).unwrap().puuid.clone(),
            participant9: match_v5.info.participants.get(9).unwrap().puuid.clone(),
            gameid: match_v5.info.game_id,
            game_creation: match_v5.info.game_creation,
            game_duration: match_v5.info.game_duration,
            game_start_timestamp: match_v5.info.game_start_timestamp,
            game_end_timestamp: match_v5.info.game_end_timestamp,
        }
    }
}
