#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodCoachDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "teamId")]
    pub team_id: String,
}

impl ValMatchV1PeriodCoachDto {
    pub fn new(puuid: String, team_id: String) -> ValMatchV1PeriodCoachDto {
        ValMatchV1PeriodCoachDto { puuid, team_id }
    }
}
