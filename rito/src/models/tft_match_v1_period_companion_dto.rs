#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftMatchV1PeriodCompanionDto {
    #[serde(rename = "skin_ID")]
    pub skin_id: i32,
    #[serde(rename = "content_ID")]
    pub content_id: String,
    #[serde(rename = "species")]
    pub species: String,
}

impl TftMatchV1PeriodCompanionDto {
    pub fn new(skin_id: i32, content_id: String, species: String) -> TftMatchV1PeriodCompanionDto {
        TftMatchV1PeriodCompanionDto {
            skin_id,
            content_id,
            species,
        }
    }
}
