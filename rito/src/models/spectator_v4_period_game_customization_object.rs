#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodGameCustomizationObject {
    /// Category identifier for Game Customization
    #[serde(rename = "category")]
    pub category: String,
    /// Game Customization content
    #[serde(rename = "content")]
    pub content: String,
}

impl SpectatorV4PeriodGameCustomizationObject {
    pub fn new(category: String, content: String) -> SpectatorV4PeriodGameCustomizationObject {
        SpectatorV4PeriodGameCustomizationObject { category, content }
    }
}
