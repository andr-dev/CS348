




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodFinishingDamageDto {
    #[serde(rename = "damageType")]
    pub damage_type: String,
    #[serde(rename = "damageItem")]
    pub damage_item: String,
    #[serde(rename = "isSecondaryFireMode")]
    pub is_secondary_fire_mode: bool,
}

impl ValMatchV1PeriodFinishingDamageDto {
    pub fn new(damage_type: String, damage_item: String, is_secondary_fire_mode: bool) -> ValMatchV1PeriodFinishingDamageDto {
        ValMatchV1PeriodFinishingDamageDto {
            damage_type,
            damage_item,
            is_secondary_fire_mode,
        }
    }
}


