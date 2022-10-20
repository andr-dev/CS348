




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodInfoDto {
    /// Unix timestamp for when the game is created on the game server (i.e., the loading screen).
    #[serde(rename = "gameCreation")]
    pub game_creation: i64,
    /// Prior to patch 11.20, this field returns the game length in milliseconds calculated from gameEndTimestamp - gameStartTimestamp. Post patch 11.20, this field returns the max timePlayed of any participant in the game in seconds, which makes the behavior of this field consistent with that of match-v4. The best way to handling the change in this field is to treat the value as milliseconds if the gameEndTimestamp field isn't in the response and to treat the value as seconds if gameEndTimestamp is in the response.
    #[serde(rename = "gameDuration")]
    pub game_duration: i64,
    /// Unix timestamp for when match ends on the game server. This timestamp can occasionally be significantly longer than when the match \"ends\". The most reliable way of determining the timestamp for the end of the match would be to add the max time played of any participant to the gameStartTimestamp. This field was added to match-v5 in patch 11.20 on Oct 5th, 2021.
    #[serde(rename = "gameEndTimestamp", skip_serializing_if = "Option::is_none")]
    pub game_end_timestamp: Option<i64>,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    /// Refer to the Game Constants documentation.
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    /// Unix timestamp for when match starts on the game server.
    #[serde(rename = "gameStartTimestamp")]
    pub game_start_timestamp: i64,
    #[serde(rename = "gameType")]
    pub game_type: String,
    /// The first two parts can be used to determine the patch a game was played on.
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    /// Refer to the Game Constants documentation.
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "participants")]
    pub participants: Vec<crate::models::MatchV5PeriodParticipantDto>,
    /// Platform where the match was played.
    #[serde(rename = "platformId")]
    pub platform_id: String,
    /// Refer to the Game Constants documentation.
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "teams")]
    pub teams: Vec<crate::models::MatchV5PeriodTeamDto>,
    /// Tournament code used to generate the match. This field was added to match-v5 in patch 11.13 on June 23rd, 2021.
    #[serde(rename = "tournamentCode", skip_serializing_if = "Option::is_none")]
    pub tournament_code: Option<String>,
}

impl MatchV5PeriodInfoDto {
    pub fn new(game_creation: i64, game_duration: i64, game_id: i64, game_mode: String, game_name: String, game_start_timestamp: i64, game_type: String, game_version: String, map_id: i32, participants: Vec<crate::models::MatchV5PeriodParticipantDto>, platform_id: String, queue_id: i32, teams: Vec<crate::models::MatchV5PeriodTeamDto>) -> MatchV5PeriodInfoDto {
        MatchV5PeriodInfoDto {
            game_creation,
            game_duration,
            game_end_timestamp: None,
            game_id,
            game_mode,
            game_name,
            game_start_timestamp,
            game_type,
            game_version,
            map_id,
            participants,
            platform_id,
            queue_id,
            teams,
            tournament_code: None,
        }
    }
}


