#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodPlayerRoundStatsDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "kills")]
    pub kills: Vec<crate::models::ValMatchV1PeriodKillDto>,
    #[serde(rename = "damage")]
    pub damage: Vec<crate::models::ValMatchV1PeriodDamageDto>,
    #[serde(rename = "score")]
    pub score: i32,
    #[serde(rename = "economy")]
    pub economy: Box<crate::models::ValMatchV1PeriodEconomyDto>,
    #[serde(rename = "ability")]
    pub ability: Box<crate::models::ValMatchV1PeriodAbilityDto>,
}

impl ValMatchV1PeriodPlayerRoundStatsDto {
    pub fn new(
        puuid: String,
        kills: Vec<crate::models::ValMatchV1PeriodKillDto>,
        damage: Vec<crate::models::ValMatchV1PeriodDamageDto>,
        score: i32,
        economy: crate::models::ValMatchV1PeriodEconomyDto,
        ability: crate::models::ValMatchV1PeriodAbilityDto,
    ) -> ValMatchV1PeriodPlayerRoundStatsDto {
        ValMatchV1PeriodPlayerRoundStatsDto {
            puuid,
            kills,
            damage,
            score,
            economy: Box::new(economy),
            ability: Box::new(ability),
        }
    }
}
