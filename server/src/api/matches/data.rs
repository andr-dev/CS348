use rocket::serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Match {
    pub id: i64,
    pub gameid: i64,
    pub platformid: String,
    pub queueid: i64,
    pub seasonid: i64,
    pub duration: i64,
    pub creation: i64,
    pub version: String,
}

impl Match {
    pub async fn get_by_gameid(pool: &Pool<Sqlite>, gameid: i64) -> Option<Match> {
        sqlx::query_as::<_, Match>("SELECT * FROM matches WHERE gameid = ?")
            .bind(gameid)
            .fetch_one(pool)
            .await
            .ok()
    }
}
