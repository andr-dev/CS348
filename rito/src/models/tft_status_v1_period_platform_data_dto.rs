#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftStatusV1PeriodPlatformDataDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "locales")]
    pub locales: Vec<String>,
    #[serde(rename = "maintenances")]
    pub maintenances: Vec<crate::models::TftStatusV1PeriodStatusDto>,
    #[serde(rename = "incidents")]
    pub incidents: Vec<crate::models::TftStatusV1PeriodStatusDto>,
}

impl TftStatusV1PeriodPlatformDataDto {
    pub fn new(
        id: String,
        name: String,
        locales: Vec<String>,
        maintenances: Vec<crate::models::TftStatusV1PeriodStatusDto>,
        incidents: Vec<crate::models::TftStatusV1PeriodStatusDto>,
    ) -> TftStatusV1PeriodPlatformDataDto {
        TftStatusV1PeriodPlatformDataDto {
            id,
            name,
            locales,
            maintenances,
            incidents,
        }
    }
}
