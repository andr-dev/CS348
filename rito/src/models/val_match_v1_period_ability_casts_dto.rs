




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodAbilityCastsDto {
    #[serde(rename = "grenadeCasts")]
    pub grenade_casts: i32,
    #[serde(rename = "ability1Casts")]
    pub ability1_casts: i32,
    #[serde(rename = "ability2Casts")]
    pub ability2_casts: i32,
    #[serde(rename = "ultimateCasts")]
    pub ultimate_casts: i32,
}

impl ValMatchV1PeriodAbilityCastsDto {
    pub fn new(grenade_casts: i32, ability1_casts: i32, ability2_casts: i32, ultimate_casts: i32) -> ValMatchV1PeriodAbilityCastsDto {
        ValMatchV1PeriodAbilityCastsDto {
            grenade_casts,
            ability1_casts,
            ability2_casts,
            ultimate_casts,
        }
    }
}


