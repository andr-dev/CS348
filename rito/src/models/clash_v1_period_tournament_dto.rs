#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClashV1PeriodTournamentDto {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "themeId")]
    pub theme_id: i32,
    #[serde(rename = "nameKey")]
    pub name_key: String,
    #[serde(rename = "nameKeySecondary")]
    pub name_key_secondary: String,
    /// Tournament phase.
    #[serde(rename = "schedule")]
    pub schedule: Vec<crate::models::ClashV1PeriodTournamentPhaseDto>,
}

impl ClashV1PeriodTournamentDto {
    pub fn new(
        id: i32,
        theme_id: i32,
        name_key: String,
        name_key_secondary: String,
        schedule: Vec<crate::models::ClashV1PeriodTournamentPhaseDto>,
    ) -> ClashV1PeriodTournamentDto {
        ClashV1PeriodTournamentDto {
            id,
            theme_id,
            name_key,
            name_key_secondary,
            schedule,
        }
    }
}
