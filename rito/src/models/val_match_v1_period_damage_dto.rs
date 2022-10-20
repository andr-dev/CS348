#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodDamageDto {
    /// PUUID
    #[serde(rename = "receiver")]
    pub receiver: String,
    #[serde(rename = "damage")]
    pub damage: i32,
    #[serde(rename = "legshots")]
    pub legshots: i32,
    #[serde(rename = "bodyshots")]
    pub bodyshots: i32,
    #[serde(rename = "headshots")]
    pub headshots: i32,
}

impl ValMatchV1PeriodDamageDto {
    pub fn new(
        receiver: String,
        damage: i32,
        legshots: i32,
        bodyshots: i32,
        headshots: i32,
    ) -> ValMatchV1PeriodDamageDto {
        ValMatchV1PeriodDamageDto {
            receiver,
            damage,
            legshots,
            bodyshots,
            headshots,
        }
    }
}
