#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorMatchV1PeriodInfoDto {
    /// (Legal values:  Constructed,  Expeditions,  Tutorial)
    #[serde(rename = "game_mode")]
    pub game_mode: GameMode,
    /// (Legal values:  Ranked,  Normal,  AI,  Tutorial,  VanillaTrial,  Singleton,  StandardGauntlet)
    #[serde(rename = "game_type")]
    pub game_type: GameType,
    #[serde(rename = "game_start_time_utc")]
    pub game_start_time_utc: String,
    #[serde(rename = "game_version")]
    pub game_version: String,
    #[serde(rename = "players")]
    pub players: Vec<crate::models::LorMatchV1PeriodPlayerDto>,
    /// Total turns taken by both players.
    #[serde(rename = "total_turn_count")]
    pub total_turn_count: i32,
}

impl LorMatchV1PeriodInfoDto {
    pub fn new(
        game_mode: GameMode,
        game_type: GameType,
        game_start_time_utc: String,
        game_version: String,
        players: Vec<crate::models::LorMatchV1PeriodPlayerDto>,
        total_turn_count: i32,
    ) -> LorMatchV1PeriodInfoDto {
        LorMatchV1PeriodInfoDto {
            game_mode,
            game_type,
            game_start_time_utc,
            game_version,
            players,
            total_turn_count,
        }
    }
}

/// (Legal values:  Constructed,  Expeditions,  Tutorial)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GameMode {
    #[serde(rename = "Constructed")]
    Constructed,
    #[serde(rename = "Expeditions")]
    Expeditions,
    #[serde(rename = "Tutorial")]
    Tutorial,
}

impl Default for GameMode {
    fn default() -> GameMode {
        Self::Constructed
    }
}
/// (Legal values:  Ranked,  Normal,  AI,  Tutorial,  VanillaTrial,  Singleton,  StandardGauntlet)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GameType {
    #[serde(rename = "Ranked")]
    Ranked,
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "Tutorial")]
    Tutorial,
    #[serde(rename = "VanillaTrial")]
    VanillaTrial,
    #[serde(rename = "Singleton")]
    Singleton,
    #[serde(rename = "StandardGauntlet")]
    StandardGauntlet,
}

impl Default for GameType {
    fn default() -> GameType {
        Self::Ranked
    }
}
