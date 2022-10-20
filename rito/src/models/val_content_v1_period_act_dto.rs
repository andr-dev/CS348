#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValContentV1PeriodActDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "localizedNames", skip_serializing_if = "Option::is_none")]
    pub localized_names: Option<Box<crate::models::ValContentV1PeriodLocalizedNamesDto>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ValContentV1PeriodActDto {
    pub fn new(name: String, id: String, is_active: bool) -> ValContentV1PeriodActDto {
        ValContentV1PeriodActDto {
            name,
            localized_names: None,
            id,
            is_active,
            parent_id: None,
            r#type: None,
        }
    }
}
