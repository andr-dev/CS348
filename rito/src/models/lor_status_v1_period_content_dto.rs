




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorStatusV1PeriodContentDto {
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "content")]
    pub content: String,
}

impl LorStatusV1PeriodContentDto {
    pub fn new(locale: String, content: String) -> LorStatusV1PeriodContentDto {
        LorStatusV1PeriodContentDto {
            locale,
            content,
        }
    }
}


