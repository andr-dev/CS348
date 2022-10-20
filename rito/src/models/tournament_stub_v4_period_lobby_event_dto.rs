




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentStubV4PeriodLobbyEventDto {
    /// The summonerId that triggered the event (Encrypted)
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    /// The type of event that was triggered
    #[serde(rename = "eventType")]
    pub event_type: String,
    /// Timestamp from the event
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl TournamentStubV4PeriodLobbyEventDto {
    pub fn new(summoner_id: String, event_type: String, timestamp: String) -> TournamentStubV4PeriodLobbyEventDto {
        TournamentStubV4PeriodLobbyEventDto {
            summoner_id,
            event_type,
            timestamp,
        }
    }
}


