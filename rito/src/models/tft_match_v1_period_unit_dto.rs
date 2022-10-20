




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftMatchV1PeriodUnitDto {
    /// A list of the unit's items. Please refer to the Teamfight Tactics documentation for item ids.
    #[serde(rename = "items")]
    pub items: Vec<i32>,
    /// This field was introduced in patch 9.22 with data_version 2.
    #[serde(rename = "character_id")]
    pub character_id: String,
    /// If a unit is chosen as part of the Fates set mechanic, the chosen trait will be indicated by this field. Otherwise this field is excluded from the response.
    #[serde(rename = "chosen", skip_serializing_if = "Option::is_none")]
    pub chosen: Option<String>,
    /// Unit name. This field is often left blank.
    #[serde(rename = "name")]
    pub name: String,
    /// Unit rarity. This doesn't equate to the unit cost.
    #[serde(rename = "rarity")]
    pub rarity: i32,
    /// Unit tier.
    #[serde(rename = "tier")]
    pub tier: i32,
}

impl TftMatchV1PeriodUnitDto {
    pub fn new(items: Vec<i32>, character_id: String, name: String, rarity: i32, tier: i32) -> TftMatchV1PeriodUnitDto {
        TftMatchV1PeriodUnitDto {
            items,
            character_id,
            chosen: None,
            name,
            rarity,
            tier,
        }
    }
}


