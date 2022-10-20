#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV4PeriodContentDto {
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "content")]
    pub content: String,
}

impl LolStatusV4PeriodContentDto {
    pub fn new(locale: String, content: String) -> LolStatusV4PeriodContentDto {
        LolStatusV4PeriodContentDto { locale, content }
    }
}
