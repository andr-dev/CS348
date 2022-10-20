




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodMatchInfoDto {
    #[serde(rename = "matchId")]
    pub match_id: String,
    #[serde(rename = "mapId")]
    pub map_id: String,
    #[serde(rename = "gameLengthMillis")]
    pub game_length_millis: i32,
    #[serde(rename = "gameStartMillis")]
    pub game_start_millis: i64,
    #[serde(rename = "provisioningFlowId")]
    pub provisioning_flow_id: String,
    #[serde(rename = "isCompleted")]
    pub is_completed: bool,
    #[serde(rename = "customGameName")]
    pub custom_game_name: String,
    #[serde(rename = "queueId")]
    pub queue_id: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "isRanked")]
    pub is_ranked: bool,
    #[serde(rename = "seasonId")]
    pub season_id: String,
}

impl ValMatchV1PeriodMatchInfoDto {
    pub fn new(match_id: String, map_id: String, game_length_millis: i32, game_start_millis: i64, provisioning_flow_id: String, is_completed: bool, custom_game_name: String, queue_id: String, game_mode: String, is_ranked: bool, season_id: String) -> ValMatchV1PeriodMatchInfoDto {
        ValMatchV1PeriodMatchInfoDto {
            match_id,
            map_id,
            game_length_millis,
            game_start_millis,
            provisioning_flow_id,
            is_completed,
            custom_game_name,
            queue_id,
            game_mode,
            is_ranked,
            season_id,
        }
    }
}


