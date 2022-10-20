




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorStatusV1PeriodPlatformDataDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "locales")]
    pub locales: Vec<String>,
    #[serde(rename = "maintenances")]
    pub maintenances: Vec<crate::models::LorStatusV1PeriodStatusDto>,
    #[serde(rename = "incidents")]
    pub incidents: Vec<crate::models::LorStatusV1PeriodStatusDto>,
}

impl LorStatusV1PeriodPlatformDataDto {
    pub fn new(id: String, name: String, locales: Vec<String>, maintenances: Vec<crate::models::LorStatusV1PeriodStatusDto>, incidents: Vec<crate::models::LorStatusV1PeriodStatusDto>) -> LorStatusV1PeriodPlatformDataDto {
        LorStatusV1PeriodPlatformDataDto {
            id,
            name,
            locales,
            maintenances,
            incidents,
        }
    }
}


