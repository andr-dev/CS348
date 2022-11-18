use rito::models::DDragonChampion;
use rocket::serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

use crate::{
    error::ServiceError,
    queries::{
        CHAMPION_GET_BY_ID_QUERY, CHAMPION_GET_BY_NAME_QUERY, CHAMPION_INSERT_QUERY,
        CHAMPION_IS_TABLE_EMPTY_QUERY, CHAMPION_WINRATE_QUERY, CHAMPION_WORST_MATCHUPS_QUERY
    },
};

#[derive(FromRow, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DbChampion {
    pub championid: i32,
    pub cname: String,
    pub title: String,
    pub blurb: String,
}

impl DbChampion {
    pub async fn get_by_name(pool: &Pool<Sqlite>, name: &String) -> Option<DbChampion> {
        sqlx::query_as::<_, DbChampion>(CHAMPION_GET_BY_NAME_QUERY)
            .bind(name)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Option<DbChampion> {
        sqlx::query_as::<_, DbChampion>(CHAMPION_GET_BY_ID_QUERY)
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn is_table_empty(pool: &Pool<Sqlite>) -> Result<bool, ServiceError> {
        Ok(sqlx::query_as::<_, (i32,)>(CHAMPION_IS_TABLE_EMPTY_QUERY)
            .fetch_one(pool)
            .await
            .map_err(|e| ServiceError { error: Box::new(e) })?
            .0
            == 1)
    }

    pub async fn insert(pool: &Pool<Sqlite>, champ: DbChampion) -> Result<(), ServiceError> {
        sqlx::query(CHAMPION_INSERT_QUERY)
            .bind(champ.championid)
            .bind(champ.cname)
            .bind(champ.title)
            .bind(champ.blurb)
            .execute(pool)
            .await
            .map_err(|e| ServiceError { error: Box::new(e) })
            .map(|_| ())
    }

    pub fn from_ddragon_champion(champ: DDragonChampion) -> Self {
        Self {
            championid: champ.key.parse().unwrap(),
            cname: champ.name,
            title: champ.title,
            blurb: champ.blurb,
        }
    }

    pub async fn get_win_rate(pool: &Pool<Sqlite>, min: u32, max: u32) -> Result<Vec<(String, f64)>, ServiceError> {
        sqlx::query_as::<_, (String, f64)>(CHAMPION_WINRATE_QUERY)
            .bind(min)
            .bind(max)
            .fetch_all(pool)
            .await
            .map_err(|e| ServiceError { error: Box::new(e) })
    }
}
