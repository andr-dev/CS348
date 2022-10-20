




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorInventoryV1PeriodCardDto {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "count")]
    pub count: String,
}

impl LorInventoryV1PeriodCardDto {
    pub fn new(code: String, count: String) -> LorInventoryV1PeriodCardDto {
        LorInventoryV1PeriodCardDto {
            code,
            count,
        }
    }
}


