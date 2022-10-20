#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentV4PeriodTournamentCodeUpdateParameters {
    /// Optional list of encrypted summonerIds in order to validate the players eligible to join the lobby. NOTE: We currently do not enforce participants at the team level, but rather the aggregate of teamOne and teamTwo. We may add the ability to enforce at the team level in the future.
    #[serde(rename = "allowedSummonerIds", skip_serializing_if = "Option::is_none")]
    pub allowed_summoner_ids: Option<Vec<String>>,
    /// The pick type              (Legal values:  BLIND_PICK,  DRAFT_MODE,  ALL_RANDOM,  TOURNAMENT_DRAFT)
    #[serde(rename = "pickType")]
    pub pick_type: PickType,
    /// The map type              (Legal values:  SUMMONERS_RIFT,  TWISTED_TREELINE,  HOWLING_ABYSS)
    #[serde(rename = "mapType")]
    pub map_type: MapType,
    /// The spectator type              (Legal values:  NONE,  LOBBYONLY,  ALL)
    #[serde(rename = "spectatorType")]
    pub spectator_type: SpectatorType,
}

impl TournamentV4PeriodTournamentCodeUpdateParameters {
    pub fn new(
        pick_type: PickType,
        map_type: MapType,
        spectator_type: SpectatorType,
    ) -> TournamentV4PeriodTournamentCodeUpdateParameters {
        TournamentV4PeriodTournamentCodeUpdateParameters {
            allowed_summoner_ids: None,
            pick_type,
            map_type,
            spectator_type,
        }
    }
}

/// The pick type              (Legal values:  BLIND_PICK,  DRAFT_MODE,  ALL_RANDOM,  TOURNAMENT_DRAFT)
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
/// The map type              (Legal values:  SUMMONERS_RIFT,  TWISTED_TREELINE,  HOWLING_ABYSS)
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
/// The spectator type              (Legal values:  NONE,  LOBBYONLY,  ALL)
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
