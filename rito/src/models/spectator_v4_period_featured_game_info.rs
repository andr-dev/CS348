




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodFeaturedGameInfo {
    /// The game mode              (Legal values:  CLASSIC,  ODIN,  ARAM,  TUTORIAL,  ONEFORALL,  ASCENSION,  FIRSTBLOOD,  KINGPORO)
    #[serde(rename = "gameMode")]
    pub game_mode: GameMode,
    /// The amount of time in seconds that has passed since the game started
    #[serde(rename = "gameLength")]
    pub game_length: i64,
    /// The ID of the map
    #[serde(rename = "mapId")]
    pub map_id: i64,
    /// The game type              (Legal values:  CUSTOM_GAME,  MATCHED_GAME,  TUTORIAL_GAME)
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    /// Banned champion information
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<crate::models::SpectatorV4PeriodBannedChampion>,
    /// The ID of the game
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "observers")]
    pub observers: Box<crate::models::SpectatorV4PeriodObserver>,
    /// The queue type (queue types are documented on the Game Constants page)
    #[serde(rename = "gameQueueConfigId")]
    pub game_queue_config_id: i64,
    /// The game start time represented in epoch milliseconds
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    /// The participant information
    #[serde(rename = "participants")]
    pub participants: Vec<crate::models::SpectatorV4PeriodParticipant>,
    /// The ID of the platform on which the game is being played
    #[serde(rename = "platformId")]
    pub platform_id: String,
}

impl SpectatorV4PeriodFeaturedGameInfo {
    pub fn new(game_mode: GameMode, game_length: i64, map_id: i64, game_type: GameType, banned_champions: Vec<crate::models::SpectatorV4PeriodBannedChampion>, game_id: i64, observers: crate::models::SpectatorV4PeriodObserver, game_queue_config_id: i64, game_start_time: i64, participants: Vec<crate::models::SpectatorV4PeriodParticipant>, platform_id: String) -> SpectatorV4PeriodFeaturedGameInfo {
        SpectatorV4PeriodFeaturedGameInfo {
            game_mode,
            game_length,
            map_id,
            game_type,
            banned_champions,
            game_id,
            observers: Box::new(observers),
            game_queue_config_id,
            game_start_time,
            participants,
            platform_id,
        }
    }
}

/// The game mode              (Legal values:  CLASSIC,  ODIN,  ARAM,  TUTORIAL,  ONEFORALL,  ASCENSION,  FIRSTBLOOD,  KINGPORO)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GameMode {
    #[serde(rename = "CLASSIC")]
    Classic,
    #[serde(rename = "ODIN")]
    Odin,
    #[serde(rename = "ARAM")]
    Aram,
    #[serde(rename = "TUTORIAL")]
    Tutorial,
    #[serde(rename = "ONEFORALL")]
    Oneforall,
    #[serde(rename = "ASCENSION")]
    Ascension,
    #[serde(rename = "FIRSTBLOOD")]
    Firstblood,
    #[serde(rename = "KINGPORO")]
    Kingporo,
}

impl Default for GameMode {
    fn default() -> GameMode {
        Self::Classic
    }
}
/// The game type              (Legal values:  CUSTOM_GAME,  MATCHED_GAME,  TUTORIAL_GAME)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GameType {
    #[serde(rename = "CUSTOM_GAME")]
    CustomGame,
    #[serde(rename = "MATCHED_GAME")]
    MatchedGame,
    #[serde(rename = "TUTORIAL_GAME")]
    TutorialGame,
}

impl Default for GameType {
    fn default() -> GameType {
        Self::CustomGame
    }
}

