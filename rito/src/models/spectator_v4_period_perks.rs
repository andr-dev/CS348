#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodPerks {
    /// IDs of the perks/runes assigned.
    #[serde(rename = "perkIds")]
    pub perk_ids: Vec<i64>,
    /// Primary runes path
    #[serde(rename = "perkStyle")]
    pub perk_style: i64,
    /// Secondary runes path
    #[serde(rename = "perkSubStyle")]
    pub perk_sub_style: i64,
}

impl SpectatorV4PeriodPerks {
    pub fn new(perk_ids: Vec<i64>, perk_style: i64, perk_sub_style: i64) -> SpectatorV4PeriodPerks {
        SpectatorV4PeriodPerks {
            perk_ids,
            perk_style,
            perk_sub_style,
        }
    }
}
