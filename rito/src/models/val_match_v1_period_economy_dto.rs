




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodEconomyDto {
    #[serde(rename = "loadoutValue")]
    pub loadout_value: i32,
    #[serde(rename = "weapon")]
    pub weapon: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "remaining")]
    pub remaining: i32,
    #[serde(rename = "spent")]
    pub spent: i32,
}

impl ValMatchV1PeriodEconomyDto {
    pub fn new(loadout_value: i32, weapon: String, armor: String, remaining: i32, spent: i32) -> ValMatchV1PeriodEconomyDto {
        ValMatchV1PeriodEconomyDto {
            loadout_value,
            weapon,
            armor,
            remaining,
            spent,
        }
    }
}


