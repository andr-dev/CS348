#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodPlayerStatsDto {
    #[serde(rename = "score")]
    pub score: i32,
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: i32,
    #[serde(rename = "kills")]
    pub kills: i32,
    #[serde(rename = "deaths")]
    pub deaths: i32,
    #[serde(rename = "assists")]
    pub assists: i32,
    #[serde(rename = "playtimeMillis")]
    pub playtime_millis: i32,
    #[serde(rename = "abilityCasts")]
    pub ability_casts: Box<crate::models::ValMatchV1PeriodAbilityCastsDto>,
}

impl ValMatchV1PeriodPlayerStatsDto {
    pub fn new(
        score: i32,
        rounds_played: i32,
        kills: i32,
        deaths: i32,
        assists: i32,
        playtime_millis: i32,
        ability_casts: crate::models::ValMatchV1PeriodAbilityCastsDto,
    ) -> ValMatchV1PeriodPlayerStatsDto {
        ValMatchV1PeriodPlayerStatsDto {
            score,
            rounds_played,
            kills,
            deaths,
            assists,
            playtime_millis,
            ability_casts: Box::new(ability_casts),
        }
    }
}
