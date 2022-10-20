




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentStubV4PeriodTournamentCodeParameters {
    /// Optional list of encrypted summonerIds in order to validate the players eligible to join the lobby. NOTE: We currently do not enforce participants at the team level, but rather the aggregate of teamOne and teamTwo. We may add the ability to enforce at the team level in the future.
    #[serde(rename = "allowedSummonerIds", skip_serializing_if = "Option::is_none")]
    pub allowed_summoner_ids: Option<Vec<String>>,
    /// Optional string that may contain any data in any format, if specified at all. Used to denote any custom information about the game.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// The team size of the game. Valid values are 1-5.
    #[serde(rename = "teamSize")]
    pub team_size: i32,
    /// The pick type of the game.              (Legal values:  BLIND_PICK,  DRAFT_MODE,  ALL_RANDOM,  TOURNAMENT_DRAFT)
    #[serde(rename = "pickType")]
    pub pick_type: PickType,
    /// The map type of the game.              (Legal values:  SUMMONERS_RIFT,  TWISTED_TREELINE,  HOWLING_ABYSS)
    #[serde(rename = "mapType")]
    pub map_type: MapType,
    /// The spectator type of the game.              (Legal values:  NONE,  LOBBYONLY,  ALL)
    #[serde(rename = "spectatorType")]
    pub spectator_type: SpectatorType,
}

impl TournamentStubV4PeriodTournamentCodeParameters {
    pub fn new(team_size: i32, pick_type: PickType, map_type: MapType, spectator_type: SpectatorType) -> TournamentStubV4PeriodTournamentCodeParameters {
        TournamentStubV4PeriodTournamentCodeParameters {
            allowed_summoner_ids: None,
            metadata: None,
            team_size,
            pick_type,
            map_type,
            spectator_type,
        }
    }
}

/// The pick type of the game.              (Legal values:  BLIND_PICK,  DRAFT_MODE,  ALL_RANDOM,  TOURNAMENT_DRAFT)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PickType {
    #[serde(rename = "BLIND_PICK")]
    BlindPick,
    #[serde(rename = "DRAFT_MODE")]
    DraftMode,
    #[serde(rename = "ALL_RANDOM")]
    AllRandom,
    #[serde(rename = "TOURNAMENT_DRAFT")]
    TournamentDraft,
}

impl Default for PickType {
    fn default() -> PickType {
        Self::BlindPick
    }
}
/// The map type of the game.              (Legal values:  SUMMONERS_RIFT,  TWISTED_TREELINE,  HOWLING_ABYSS)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MapType {
    #[serde(rename = "SUMMONERS_RIFT")]
    SummonersRift,
    #[serde(rename = "TWISTED_TREELINE")]
    TwistedTreeline,
    #[serde(rename = "HOWLING_ABYSS")]
    HowlingAbyss,
}

impl Default for MapType {
    fn default() -> MapType {
        Self::SummonersRift
    }
}
/// The spectator type of the game.              (Legal values:  NONE,  LOBBYONLY,  ALL)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpectatorType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "LOBBYONLY")]
    Lobbyonly,
    #[serde(rename = "ALL")]
    All,
}

impl Default for SpectatorType {
    fn default() -> SpectatorType {
        Self::None
    }
}

