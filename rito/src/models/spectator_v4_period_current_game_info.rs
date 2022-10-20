




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodCurrentGameInfo {
    /// The ID of the game
    #[serde(rename = "gameId")]
    pub game_id: i64,
    /// The game type
    #[serde(rename = "gameType")]
    pub game_type: String,
    /// The game start time represented in epoch milliseconds
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    /// The ID of the map
    #[serde(rename = "mapId")]
    pub map_id: i64,
    /// The amount of time in seconds that has passed since the game started
    #[serde(rename = "gameLength")]
    pub game_length: i64,
    /// The ID of the platform on which the game is being played
    #[serde(rename = "platformId")]
    pub platform_id: String,
    /// The game mode
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    /// Banned champion information
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<crate::models::SpectatorV4PeriodBannedChampion>,
    /// The queue type (queue types are documented on the Game Constants page)
    #[serde(rename = "gameQueueConfigId", skip_serializing_if = "Option::is_none")]
    pub game_queue_config_id: Option<i64>,
    #[serde(rename = "observers")]
    pub observers: Box<crate::models::SpectatorV4PeriodObserver>,
    /// The participant information
    #[serde(rename = "participants")]
    pub participants: Vec<crate::models::SpectatorV4PeriodCurrentGameParticipant>,
}

impl SpectatorV4PeriodCurrentGameInfo {
    pub fn new(game_id: i64, game_type: String, game_start_time: i64, map_id: i64, game_length: i64, platform_id: String, game_mode: String, banned_champions: Vec<crate::models::SpectatorV4PeriodBannedChampion>, observers: crate::models::SpectatorV4PeriodObserver, participants: Vec<crate::models::SpectatorV4PeriodCurrentGameParticipant>) -> SpectatorV4PeriodCurrentGameInfo {
        SpectatorV4PeriodCurrentGameInfo {
            game_id,
            game_type,
            game_start_time,
            map_id,
            game_length,
            platform_id,
            game_mode,
            banned_champions,
            game_queue_config_id: None,
            observers: Box::new(observers),
            participants,
        }
    }
}


