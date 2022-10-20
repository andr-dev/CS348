#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodMatchlistDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "history")]
    pub history: Vec<crate::models::ValMatchV1PeriodMatchlistEntryDto>,
}

impl ValMatchV1PeriodMatchlistDto {
    pub fn new(
        puuid: String,
        history: Vec<crate::models::ValMatchV1PeriodMatchlistEntryDto>,
    ) -> ValMatchV1PeriodMatchlistDto {
        ValMatchV1PeriodMatchlistDto { puuid, history }
    }
}
