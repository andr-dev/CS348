use rito::models::DDragonChampion;
use rocket::serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

use crate::{
    error::ServiceError,
    queries::{CHAMPION_GET_BY_ID_QUERY, CHAMPION_GET_BY_NAME_QUERY},
};

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Champion {
    pub championid: i32,
    pub cname: String,
    pub title: String,
    pub blurb: String,
}

impl Champion {
    pub async fn get_by_name(pool: &Pool<Sqlite>, name: &String) -> Option<Champion> {
        sqlx::query_as::<_, Champion>(CHAMPION_GET_BY_NAME_QUERY)
            .bind(name)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Option<Champion> {
        sqlx::query_as::<_, Champion>(CHAMPION_GET_BY_ID_QUERY)
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn is_table_empty(pool: &Pool<Sqlite>) -> Result<bool, ServiceError> {
        Ok(sqlx::query_as::<_, (i32,)>(
            "SELECT CASE WHEN EXISTS(SELECT 1 FROM champions) THEN 0 ELSE 1 END AS IsEmpty",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServiceError { error: Box::new(e) })?
        .0 == 1)
    }

    pub async fn insert(pool: &Pool<Sqlite>, champ: Champion) -> Result<(), ServiceError> {
        sqlx::query("INSERT INTO champions VALUES (?, ?, ?, ?)")
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
}
