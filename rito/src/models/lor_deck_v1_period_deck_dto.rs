#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorDeckV1PeriodDeckDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "code")]
    pub code: String,
}

impl LorDeckV1PeriodDeckDto {
    pub fn new(id: String, name: String, code: String) -> LorDeckV1PeriodDeckDto {
        LorDeckV1PeriodDeckDto { id, name, code }
    }
}
