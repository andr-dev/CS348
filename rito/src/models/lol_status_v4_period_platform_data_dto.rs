#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV4PeriodPlatformDataDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "locales")]
    pub locales: Vec<String>,
    #[serde(rename = "maintenances")]
    pub maintenances: Vec<crate::models::LolStatusV4PeriodStatusDto>,
    #[serde(rename = "incidents")]
    pub incidents: Vec<crate::models::LolStatusV4PeriodStatusDto>,
}

impl LolStatusV4PeriodPlatformDataDto {
    pub fn new(
        id: String,
        name: String,
        locales: Vec<String>,
        maintenances: Vec<crate::models::LolStatusV4PeriodStatusDto>,
        incidents: Vec<crate::models::LolStatusV4PeriodStatusDto>,
    ) -> LolStatusV4PeriodPlatformDataDto {
        LolStatusV4PeriodPlatformDataDto {
            id,
            name,
            locales,
            maintenances,
            incidents,
        }
    }
}
