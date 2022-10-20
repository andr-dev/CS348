#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValStatusV1PeriodPlatformDataDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "locales")]
    pub locales: Vec<String>,
    #[serde(rename = "maintenances")]
    pub maintenances: Vec<crate::models::ValStatusV1PeriodStatusDto>,
    #[serde(rename = "incidents")]
    pub incidents: Vec<crate::models::ValStatusV1PeriodStatusDto>,
}

impl ValStatusV1PeriodPlatformDataDto {
    pub fn new(
        id: String,
        name: String,
        locales: Vec<String>,
        maintenances: Vec<crate::models::ValStatusV1PeriodStatusDto>,
        incidents: Vec<crate::models::ValStatusV1PeriodStatusDto>,
    ) -> ValStatusV1PeriodPlatformDataDto {
        ValStatusV1PeriodPlatformDataDto {
            id,
            name,
            locales,
            maintenances,
            incidents,
        }
    }
}
