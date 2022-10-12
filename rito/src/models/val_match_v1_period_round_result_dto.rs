/*
 * Riot API
 *
 *  OpenAPI/Swagger version of the [Riot API](https://developer.riotgames.com/). Automatically generated daily. ## OpenAPI Spec File The following versions of the Riot API spec file are available: - `openapi-3.0.0.json` ([view file](../openapi-3.0.0.json), [ui select](?url=../openapi-3.0.0.json)) - `openapi-3.0.0.min.json` ([view file](../openapi-3.0.0.min.json), [ui select](?url=../openapi-3.0.0.min.json)) - `openapi-3.0.0.yml` ([view file](../openapi-3.0.0.yml), [ui select](?url=../openapi-3.0.0.yml)) - `openapi-3.0.0.min.yml` ([view file](../openapi-3.0.0.min.yml), [ui select](?url=../openapi-3.0.0.min.yml)) - `swaggerspec-2.0.json` ([view file](../swaggerspec-2.0.json), [ui select](?url=../swaggerspec-2.0.json)) - `swaggerspec-2.0.min.json` ([view file](../swaggerspec-2.0.min.json), [ui select](?url=../swaggerspec-2.0.min.json)) - `swaggerspec-2.0.yml` ([view file](../swaggerspec-2.0.yml), [ui select](?url=../swaggerspec-2.0.yml)) - `swaggerspec-2.0.min.yml` ([view file](../swaggerspec-2.0.min.yml), [ui select](?url=../swaggerspec-2.0.min.yml)) ## Other Files - Missing DTOs: [`missing.json`](../missing.json) - [Enum Files](../enums/) ## Source Code Source code on [GitHub](https://github.com/MingweiSamuel/riotapi-schema). Pull requests welcome! ## Automatically Generated Rebuilt on [Travis CI](https://travis-ci.com/MingweiSamuel/riotapi-schema/builds) daily. *** 
 *
 * The version of the OpenAPI document: dae26e2703c82eb19447d1b27f1209801cb83beb
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodRoundResultDto {
    #[serde(rename = "roundNum")]
    pub round_num: i32,
    #[serde(rename = "roundResult")]
    pub round_result: String,
    #[serde(rename = "roundCeremony")]
    pub round_ceremony: String,
    #[serde(rename = "winningTeam")]
    pub winning_team: String,
    /// PUUID of player
    #[serde(rename = "bombPlanter")]
    pub bomb_planter: String,
    /// PUUID of player
    #[serde(rename = "bombDefuser")]
    pub bomb_defuser: String,
    #[serde(rename = "plantRoundTime")]
    pub plant_round_time: i32,
    #[serde(rename = "plantPlayerLocations")]
    pub plant_player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>,
    #[serde(rename = "plantLocation")]
    pub plant_location: Box<crate::models::ValMatchV1PeriodLocationDto>,
    #[serde(rename = "plantSite")]
    pub plant_site: String,
    #[serde(rename = "defuseRoundTime")]
    pub defuse_round_time: i32,
    #[serde(rename = "defusePlayerLocations")]
    pub defuse_player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>,
    #[serde(rename = "defuseLocation")]
    pub defuse_location: Box<crate::models::ValMatchV1PeriodLocationDto>,
    #[serde(rename = "playerStats")]
    pub player_stats: Vec<crate::models::ValMatchV1PeriodPlayerRoundStatsDto>,
    #[serde(rename = "roundResultCode")]
    pub round_result_code: String,
}

impl ValMatchV1PeriodRoundResultDto {
    pub fn new(round_num: i32, round_result: String, round_ceremony: String, winning_team: String, bomb_planter: String, bomb_defuser: String, plant_round_time: i32, plant_player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>, plant_location: crate::models::ValMatchV1PeriodLocationDto, plant_site: String, defuse_round_time: i32, defuse_player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>, defuse_location: crate::models::ValMatchV1PeriodLocationDto, player_stats: Vec<crate::models::ValMatchV1PeriodPlayerRoundStatsDto>, round_result_code: String) -> ValMatchV1PeriodRoundResultDto {
        ValMatchV1PeriodRoundResultDto {
            round_num,
            round_result,
            round_ceremony,
            winning_team,
            bomb_planter,
            bomb_defuser,
            plant_round_time,
            plant_player_locations,
            plant_location: Box::new(plant_location),
            plant_site,
            defuse_round_time,
            defuse_player_locations,
            defuse_location: Box::new(defuse_location),
            player_stats,
            round_result_code,
        }
    }
}


