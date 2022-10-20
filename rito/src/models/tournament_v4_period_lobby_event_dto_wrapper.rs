




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentV4PeriodLobbyEventDtoWrapper {
    #[serde(rename = "eventList")]
    pub event_list: Vec<crate::models::TournamentV4PeriodLobbyEventDto>,
}

impl TournamentV4PeriodLobbyEventDtoWrapper {
    pub fn new(event_list: Vec<crate::models::TournamentV4PeriodLobbyEventDto>) -> TournamentV4PeriodLobbyEventDtoWrapper {
        TournamentV4PeriodLobbyEventDtoWrapper {
            event_list,
        }
    }
}


