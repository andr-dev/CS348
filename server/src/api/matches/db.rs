use rito::models::{MatchV5PeriodMatchDto, MatchV5PeriodParticipantDto};
use rocket::serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

use crate::{
    api::{summoner::DbSummoner, DbChampion},
    error::ServiceError,
    queries::{
        MATCH_FIND_BY_ID_QUERY, MATCH_INSERT_QUERY, SUMMONER_MATCHES_INSERT_QUERY,
        SUMMONER_MATCHES_KDA_QUERY,
    },
};

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
        let r = sqlx::query_as::<_, DbMatch>(MATCH_FIND_BY_ID_QUERY)
            .bind(match_id)
            .fetch_one(pool)
            .await;

        println!("R: {:?}", r);

        r.ok()
    }

    pub async fn insert_match(pool: &Pool<Sqlite>, db_match: &Self) -> Result<(), ServiceError> {
        sqlx::query(MATCH_INSERT_QUERY)
            .bind(db_match.matchid.clone())
            .bind(db_match.data_version.clone())
            .bind(db_match.participant0.clone())
            .bind(db_match.participant1.clone())
            .bind(db_match.participant2.clone())
            .bind(db_match.participant3.clone())
            .bind(db_match.participant4.clone())
            .bind(db_match.participant5.clone())
            .bind(db_match.participant6.clone())
            .bind(db_match.participant7.clone())
            .bind(db_match.participant8.clone())
            .bind(db_match.participant9.clone())
            .bind(db_match.gameid)
            .bind(db_match.game_creation)
            .bind(db_match.game_duration)
            .bind(db_match.game_start_timestamp)
            .bind(db_match.game_end_timestamp)
            .execute(pool)
            .await
            .map_err(|e| ServiceError { error: Box::new(e) })
            .map(|_| ())
    }

    pub async fn get_kda_by_puuid(
        pool: &Pool<Sqlite>,
        puuid: &str,
    ) -> Result<(f64, f64, f64), ServiceError> {
        sqlx::query_as::<_, (f64, f64, f64)>(SUMMONER_MATCHES_KDA_QUERY)
            .bind(puuid)
            .fetch_one(pool)
            .await
            .map_err(|e| ServiceError { error: Box::new(e) })
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

#[derive(FromRow, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DbSummonerMatch {
    pub puuid: String,
    pub matchid: String,

    pub championid: i32,
    pub win: bool,
    pub team_position: String,
    pub champ_level: i32,
    pub champ_experience: i32,
    pub vision_score: i32,
    pub gold_earned: i32,
    pub gold_spent: i32,

    pub kills: i32,
    pub double_kills: i32,
    pub triple_kills: i32,
    pub quadra_kills: i32,
    pub penta_kills: i32,
    pub deaths: i32,
    pub assists: i32,

    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,

    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,

    pub dragon_kills: i32,
    pub baron_kills: i32,

    pub turret_kills: i32,
    pub turret_takedowns: i32,
    pub turrets_lost: i32,

    pub inhibitor_kills: i32,
    pub inhibitor_takedowns: i32,
    pub inhibitors_lost: i32,

    pub damage_dealt_to_buildings: i32,
    pub damage_dealt_to_objectives: i32,
    pub damage_dealt_to_turrets: i32,

    pub consumables_purchased: i32,
    pub wards_placed: i32,
    pub wards_killed: i32,
}

impl DbSummonerMatch {
    pub async fn insert_summoner_match(
        pool: &Pool<Sqlite>,
        dbsm: Self,
    ) -> Result<(), ServiceError> {
        sqlx::query(SUMMONER_MATCHES_INSERT_QUERY)
            .bind(dbsm.puuid.clone())
            .bind(dbsm.matchid.clone())
            .bind(dbsm.championid)
            .bind(dbsm.win)
            .bind(dbsm.team_position)
            .bind(dbsm.champ_level)
            .bind(dbsm.champ_experience)
            .bind(dbsm.vision_score)
            .bind(dbsm.gold_earned)
            .bind(dbsm.gold_spent)
            .bind(dbsm.kills)
            .bind(dbsm.double_kills)
            .bind(dbsm.triple_kills)
            .bind(dbsm.quadra_kills)
            .bind(dbsm.penta_kills)
            .bind(dbsm.deaths)
            .bind(dbsm.assists)
            .bind(dbsm.item0)
            .bind(dbsm.item1)
            .bind(dbsm.item2)
            .bind(dbsm.item3)
            .bind(dbsm.item4)
            .bind(dbsm.item5)
            .bind(dbsm.item6)
            .bind(dbsm.first_blood_assist)
            .bind(dbsm.first_blood_kill)
            .bind(dbsm.first_tower_assist)
            .bind(dbsm.first_tower_kill)
            .bind(dbsm.dragon_kills)
            .bind(dbsm.baron_kills)
            .bind(dbsm.turret_kills)
            .bind(dbsm.turret_takedowns)
            .bind(dbsm.turrets_lost)
            .bind(dbsm.inhibitor_kills)
            .bind(dbsm.inhibitor_takedowns)
            .bind(dbsm.inhibitors_lost)
            .bind(dbsm.damage_dealt_to_buildings)
            .bind(dbsm.damage_dealt_to_objectives)
            .bind(dbsm.damage_dealt_to_turrets)
            .bind(dbsm.consumables_purchased)
            .bind(dbsm.wards_placed)
            .bind(dbsm.wards_killed)
            .execute(pool)
            .await
            .map_err(|e| ServiceError { error: Box::new(e) })
            .map(|_| ())
    }

    pub async fn from_match_v5_participant(
        participant: MatchV5PeriodParticipantDto,
        db_match: &DbMatch,
    ) -> Self {
        Self {
            puuid: participant.puuid,
            matchid: db_match.matchid.clone(),

            championid: participant.champion_id,
            win: participant.win,
            team_position: participant.team_position,
            champ_level: participant.champ_level,
            champ_experience: participant.champ_experience,
            vision_score: participant.vision_score,
            gold_earned: participant.gold_earned,
            gold_spent: participant.gold_spent,

            kills: participant.kills,
            double_kills: participant.double_kills,
            triple_kills: participant.triple_kills,
            quadra_kills: participant.quadra_kills,
            penta_kills: participant.penta_kills,
            deaths: participant.deaths,
            assists: participant.assists,

            item0: participant.item0,
            item1: participant.item1,
            item2: participant.item2,
            item3: participant.item3,
            item4: participant.item4,
            item5: participant.item5,
            item6: participant.item6,

            first_blood_assist: participant.first_blood_assist,
            first_blood_kill: participant.first_blood_kill,
            first_tower_assist: participant.first_tower_assist,
            first_tower_kill: participant.first_tower_kill,

            dragon_kills: participant.dragon_kills,
            baron_kills: participant.baron_kills,

            turret_kills: participant.turret_kills,
            turret_takedowns: participant.turret_takedowns,
            turrets_lost: participant.turrets_lost,

            inhibitor_kills: participant.inhibitor_kills,
            inhibitor_takedowns: participant.inhibitor_takedowns,
            inhibitors_lost: participant.inhibitors_lost,

            damage_dealt_to_buildings: participant.damage_dealt_to_buildings,
            damage_dealt_to_objectives: participant.damage_dealt_to_objectives,
            damage_dealt_to_turrets: participant.damage_dealt_to_turrets,

            consumables_purchased: participant.consumables_purchased,
            wards_placed: participant.wards_placed,
            wards_killed: participant.wards_killed,
        }
    }
}
