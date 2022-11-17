use chrono::Utc;
use rito::models::SummonerV4PeriodSummonerDto;
use rocket::serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Error;
use sqlx::FromRow;
use sqlx::Pool;
use sqlx::Sqlite;

use crate::queries::SUMMONER_FIND_BY_PUUID_QUERY;
use crate::queries::SUMMONER_GET_BY_NAME_QUERY;
use crate::queries::SUMMONER_INSERT_QUERY;
use crate::queries::SUMMONER_UPDATE_QUERY;
use crate::queries::SUMMONER_MATCH_IDS_BY_PUUID_QUERY;

#[derive(FromRow, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DbSummoner {
    pub accountid: String,
    pub profileiconid: i32,
    pub lastupdate: NaiveDateTime,
    pub sname: String,
    pub id: String,
    pub puuid: String,
    pub summonerlevel: i64,
}

impl DbSummoner {
    pub async fn get_by_summoner_puuid(pool: &Pool<Sqlite>, puuid: &String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        sqlx::query_as::<_, DbSummoner>(SUMMONER_FIND_BY_PUUID_QUERY)
            .bind(puuid)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_summoner_name(pool: &Pool<Sqlite>, name: &String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        sqlx::query_as::<_, DbSummoner>(SUMMONER_GET_BY_NAME_QUERY)
            .bind(name)
            .fetch_one(pool)
            .await
    }

    pub async fn insert_summoner(pool: &Pool<Sqlite>, summoner: &DbSummoner) -> Result<(), Error> {
        sqlx::query(SUMMONER_INSERT_QUERY)
            .bind(summoner.accountid.clone())
            .bind(summoner.profileiconid.clone())
            .bind(summoner.lastupdate.clone())
            .bind(summoner.sname.clone())
            .bind(summoner.id.clone())
            .bind(summoner.puuid.clone())
            .bind(summoner.summonerlevel.clone())
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub async fn update_summoner(pool: &Pool<Sqlite>, summoner: &DbSummoner) -> Result<(), Error> {
        println!("SUMMONER: {:?}", summoner);

        sqlx::query(SUMMONER_UPDATE_QUERY)
            .bind(summoner.accountid.clone())
            .bind(summoner.puuid.clone())
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub fn from_summoner_v4(summoner: SummonerV4PeriodSummonerDto) -> Self
    where
        Self: Sized,
    {
        DbSummoner {
            accountid: summoner.account_id,
            profileiconid: summoner.profile_icon_id,
            lastupdate: Utc::now().naive_utc(),
            sname: summoner.name,
            id: summoner.id,
            puuid: summoner.puuid,
            summonerlevel: summoner.summoner_level,
        }
    }

    pub async fn get_match_ids_by_puuid(pool: &Pool<Sqlite>, puuid: &String) -> Result<Vec<String>, Error> {
        sqlx::query_as::<_, (String,)>(SUMMONER_MATCH_IDS_BY_PUUID_QUERY)
            .bind(puuid)
            .fetch_all(pool)
            .await
            .map(|v| v.iter().map(|x| x.0.clone()).collect::<Vec<String>>())
    }
}
