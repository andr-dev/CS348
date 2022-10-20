#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentV4PeriodTournamentRegistrationParameters {
    /// The provider ID to specify the regional registered provider data to associate this tournament.
    #[serde(rename = "providerId")]
    pub provider_id: i32,
    /// The optional name of the tournament.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TournamentV4PeriodTournamentRegistrationParameters {
    pub fn new(provider_id: i32) -> TournamentV4PeriodTournamentRegistrationParameters {
        TournamentV4PeriodTournamentRegistrationParameters {
            provider_id,
            name: None,
        }
    }
}
