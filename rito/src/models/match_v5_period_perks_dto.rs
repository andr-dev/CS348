




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodPerksDto {
    #[serde(rename = "statPerks")]
    pub stat_perks: Box<crate::models::MatchV5PeriodPerkStatsDto>,
    #[serde(rename = "styles")]
    pub styles: Vec<crate::models::MatchV5PeriodPerkStyleDto>,
}

impl MatchV5PeriodPerksDto {
    pub fn new(stat_perks: crate::models::MatchV5PeriodPerkStatsDto, styles: Vec<crate::models::MatchV5PeriodPerkStyleDto>) -> MatchV5PeriodPerksDto {
        MatchV5PeriodPerksDto {
            stat_perks: Box::new(stat_perks),
            styles,
        }
    }
}


