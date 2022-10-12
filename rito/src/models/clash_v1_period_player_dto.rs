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
pub struct ClashV1PeriodPlayerDto {
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// (Legal values:  UNSELECTED,  FILL,  TOP,  JUNGLE,  MIDDLE,  BOTTOM,  UTILITY)
    #[serde(rename = "position")]
    pub position: Position,
    /// (Legal values:  CAPTAIN,  MEMBER)
    #[serde(rename = "role")]
    pub role: Role,
}

impl ClashV1PeriodPlayerDto {
    pub fn new(summoner_id: String, position: Position, role: Role) -> ClashV1PeriodPlayerDto {
        ClashV1PeriodPlayerDto {
            summoner_id,
            team_id: None,
            position,
            role,
        }
    }
}

/// (Legal values:  UNSELECTED,  FILL,  TOP,  JUNGLE,  MIDDLE,  BOTTOM,  UTILITY)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Position {
    #[serde(rename = "UNSELECTED")]
    Unselected,
    #[serde(rename = "FILL")]
    Fill,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "JUNGLE")]
    Jungle,
    #[serde(rename = "MIDDLE")]
    Middle,
    #[serde(rename = "BOTTOM")]
    Bottom,
    #[serde(rename = "UTILITY")]
    Utility,
}

impl Default for Position {
    fn default() -> Position {
        Self::Unselected
    }
}
/// (Legal values:  CAPTAIN,  MEMBER)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "CAPTAIN")]
    Captain,
    #[serde(rename = "MEMBER")]
    Member,
}

impl Default for Role {
    fn default() -> Role {
        Self::Captain
    }
}

