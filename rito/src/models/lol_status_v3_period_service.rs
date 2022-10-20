#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV3PeriodService {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "incidents")]
    pub incidents: Vec<crate::models::LolStatusV3PeriodIncident>,
}

impl LolStatusV3PeriodService {
    pub fn new(
        name: String,
        slug: String,
        status: String,
        incidents: Vec<crate::models::LolStatusV3PeriodIncident>,
    ) -> LolStatusV3PeriodService {
        LolStatusV3PeriodService {
            name,
            slug,
            status,
            incidents,
        }
    }
}
