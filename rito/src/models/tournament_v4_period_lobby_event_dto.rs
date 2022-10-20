




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentV4PeriodLobbyEventDto {
    /// Timestamp from the event
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// The type of event that was triggered
    #[serde(rename = "eventType")]
    pub event_type: String,
    /// The summonerId that triggered the event (Encrypted)
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
}

impl TournamentV4PeriodLobbyEventDto {
    pub fn new(timestamp: String, event_type: String, summoner_id: String) -> TournamentV4PeriodLobbyEventDto {
        TournamentV4PeriodLobbyEventDto {
            timestamp,
            event_type,
            summoner_id,
        }
    }
}


