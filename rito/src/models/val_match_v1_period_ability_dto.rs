




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodAbilityDto {
    #[serde(rename = "grenadeEffects")]
    pub grenade_effects: String,
    #[serde(rename = "ability1Effects")]
    pub ability1_effects: String,
    #[serde(rename = "ability2Effects")]
    pub ability2_effects: String,
    #[serde(rename = "ultimateEffects")]
    pub ultimate_effects: String,
}

impl ValMatchV1PeriodAbilityDto {
    pub fn new(grenade_effects: String, ability1_effects: String, ability2_effects: String, ultimate_effects: String) -> ValMatchV1PeriodAbilityDto {
        ValMatchV1PeriodAbilityDto {
            grenade_effects,
            ability1_effects,
            ability2_effects,
            ultimate_effects,
        }
    }
}


