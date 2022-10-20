#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV3PeriodIncident {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updates")]
    pub updates: Vec<crate::models::LolStatusV3PeriodMessage>,
}

impl LolStatusV3PeriodIncident {
    pub fn new(
        id: i64,
        active: bool,
        created_at: String,
        updates: Vec<crate::models::LolStatusV3PeriodMessage>,
    ) -> LolStatusV3PeriodIncident {
        LolStatusV3PeriodIncident {
            id,
            active,
            created_at,
            updates,
        }
    }
}
