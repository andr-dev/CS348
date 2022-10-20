




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClashV1PeriodTournamentPhaseDto {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "registrationTime")]
    pub registration_time: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "cancelled")]
    pub cancelled: bool,
}

impl ClashV1PeriodTournamentPhaseDto {
    pub fn new(id: i32, registration_time: i64, start_time: i64, cancelled: bool) -> ClashV1PeriodTournamentPhaseDto {
        ClashV1PeriodTournamentPhaseDto {
            id,
            registration_time,
            start_time,
            cancelled,
        }
    }
}


