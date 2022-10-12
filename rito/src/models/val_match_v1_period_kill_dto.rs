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
pub struct ValMatchV1PeriodKillDto {
    #[serde(rename = "timeSinceGameStartMillis")]
    pub time_since_game_start_millis: i32,
    #[serde(rename = "timeSinceRoundStartMillis")]
    pub time_since_round_start_millis: i32,
    /// PUUID
    #[serde(rename = "killer")]
    pub killer: String,
    /// PUUID
    #[serde(rename = "victim")]
    pub victim: String,
    #[serde(rename = "victimLocation")]
    pub victim_location: Box<crate::models::ValMatchV1PeriodLocationDto>,
    /// List of PUUIDs
    #[serde(rename = "assistants")]
    pub assistants: Vec<String>,
    #[serde(rename = "playerLocations")]
    pub player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>,
    #[serde(rename = "finishingDamage")]
    pub finishing_damage: Box<crate::models::ValMatchV1PeriodFinishingDamageDto>,
}

impl ValMatchV1PeriodKillDto {
    pub fn new(time_since_game_start_millis: i32, time_since_round_start_millis: i32, killer: String, victim: String, victim_location: crate::models::ValMatchV1PeriodLocationDto, assistants: Vec<String>, player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>, finishing_damage: crate::models::ValMatchV1PeriodFinishingDamageDto) -> ValMatchV1PeriodKillDto {
        ValMatchV1PeriodKillDto {
            time_since_game_start_millis,
            time_since_round_start_millis,
            killer,
            victim,
            victim_location: Box::new(victim_location),
            assistants,
            player_locations,
            finishing_damage: Box::new(finishing_damage),
        }
    }
}


