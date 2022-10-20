#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValStatusV1PeriodContentDto {
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "content")]
    pub content: String,
}

impl ValStatusV1PeriodContentDto {
    pub fn new(locale: String, content: String) -> ValStatusV1PeriodContentDto {
        ValStatusV1PeriodContentDto { locale, content }
    }
}
