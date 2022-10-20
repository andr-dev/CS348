#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentStubV4PeriodLobbyEventDtoWrapper {
    #[serde(rename = "eventList")]
    pub event_list: Vec<crate::models::TournamentStubV4PeriodLobbyEventDto>,
}

impl TournamentStubV4PeriodLobbyEventDtoWrapper {
    pub fn new(
        event_list: Vec<crate::models::TournamentStubV4PeriodLobbyEventDto>,
    ) -> TournamentStubV4PeriodLobbyEventDtoWrapper {
        TournamentStubV4PeriodLobbyEventDtoWrapper { event_list }
    }
}
