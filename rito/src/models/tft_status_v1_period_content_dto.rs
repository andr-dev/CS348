




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftStatusV1PeriodContentDto {
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "content")]
    pub content: String,
}

impl TftStatusV1PeriodContentDto {
    pub fn new(locale: String, content: String) -> TftStatusV1PeriodContentDto {
        TftStatusV1PeriodContentDto {
            locale,
            content,
        }
    }
}


