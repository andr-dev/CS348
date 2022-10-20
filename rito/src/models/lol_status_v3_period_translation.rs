




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV3PeriodTranslation {
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "content")]
    pub content: String,
}

impl LolStatusV3PeriodTranslation {
    pub fn new(updated_at: String, locale: String, content: String) -> LolStatusV3PeriodTranslation {
        LolStatusV3PeriodTranslation {
            updated_at,
            locale,
            content,
        }
    }
}


