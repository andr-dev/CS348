#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftMatchV1PeriodTraitDto {
    /// Trait name.
    #[serde(rename = "name")]
    pub name: String,
    /// Number of units with this trait.
    #[serde(rename = "num_units")]
    pub num_units: i32,
    /// Current style for this trait. (0 = No style, 1 = Bronze, 2 = Silver, 3 = Gold, 4 = Chromatic)
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<i32>,
    /// Current active tier for the trait.
    #[serde(rename = "tier_current")]
    pub tier_current: i32,
    /// Total tiers for the trait.
    #[serde(rename = "tier_total", skip_serializing_if = "Option::is_none")]
    pub tier_total: Option<i32>,
}

impl TftMatchV1PeriodTraitDto {
    pub fn new(name: String, num_units: i32, tier_current: i32) -> TftMatchV1PeriodTraitDto {
        TftMatchV1PeriodTraitDto {
            name,
            num_units,
            style: None,
            tier_current,
            tier_total: None,
        }
    }
}
