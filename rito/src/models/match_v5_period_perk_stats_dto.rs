#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodPerkStatsDto {
    #[serde(rename = "defense")]
    pub defense: i32,
    #[serde(rename = "flex")]
    pub flex: i32,
    #[serde(rename = "offense")]
    pub offense: i32,
}

impl MatchV5PeriodPerkStatsDto {
    pub fn new(defense: i32, flex: i32, offense: i32) -> MatchV5PeriodPerkStatsDto {
        MatchV5PeriodPerkStatsDto {
            defense,
            flex,
            offense,
        }
    }
}
