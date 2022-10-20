




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV3PeriodMessage {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "heading")]
    pub heading: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "translations")]
    pub translations: Vec<crate::models::LolStatusV3PeriodTranslation>,
}

impl LolStatusV3PeriodMessage {
    pub fn new(id: String, author: String, heading: String, content: String, severity: String, created_at: String, updated_at: String, translations: Vec<crate::models::LolStatusV3PeriodTranslation>) -> LolStatusV3PeriodMessage {
        LolStatusV3PeriodMessage {
            id,
            author,
            heading,
            content,
            severity,
            created_at,
            updated_at,
            translations,
        }
    }
}


