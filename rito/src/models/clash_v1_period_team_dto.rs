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
pub struct ClashV1PeriodTeamDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "iconId")]
    pub icon_id: i32,
    #[serde(rename = "tier")]
    pub tier: i32,
    /// Summoner ID of the team captain.
    #[serde(rename = "captain")]
    pub captain: String,
    #[serde(rename = "abbreviation")]
    pub abbreviation: String,
    /// Team members.
    #[serde(rename = "players")]
    pub players: Vec<crate::models::ClashV1PeriodPlayerDto>,
}

impl ClashV1PeriodTeamDto {
    pub fn new(id: String, tournament_id: i32, name: String, icon_id: i32, tier: i32, captain: String, abbreviation: String, players: Vec<crate::models::ClashV1PeriodPlayerDto>) -> ClashV1PeriodTeamDto {
        ClashV1PeriodTeamDto {
            id,
            tournament_id,
            name,
            icon_id,
            tier,
            captain,
            abbreviation,
            players,
        }
    }
}


