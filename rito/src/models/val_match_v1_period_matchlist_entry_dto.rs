




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodMatchlistEntryDto {
    #[serde(rename = "matchId")]
    pub match_id: String,
    #[serde(rename = "gameStartTimeMillis")]
    pub game_start_time_millis: i64,
    #[serde(rename = "teamId")]
    pub team_id: String,
}

impl ValMatchV1PeriodMatchlistEntryDto {
    pub fn new(match_id: String, game_start_time_millis: i64, team_id: String) -> ValMatchV1PeriodMatchlistEntryDto {
        ValMatchV1PeriodMatchlistEntryDto {
            match_id,
            game_start_time_millis,
            team_id,
        }
    }
}


