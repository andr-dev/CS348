#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValContentV1PeriodContentItemDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "localizedNames", skip_serializing_if = "Option::is_none")]
    pub localized_names: Option<Box<crate::models::ValContentV1PeriodLocalizedNamesDto>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "assetName")]
    pub asset_name: String,
    /// This field is only included for maps and game modes. These values are used in the match response.
    #[serde(rename = "assetPath", skip_serializing_if = "Option::is_none")]
    pub asset_path: Option<String>,
}

impl ValContentV1PeriodContentItemDto {
    pub fn new(name: String, id: String, asset_name: String) -> ValContentV1PeriodContentItemDto {
        ValContentV1PeriodContentItemDto {
            name,
            localized_names: None,
            id,
            asset_name,
            asset_path: None,
        }
    }
}
