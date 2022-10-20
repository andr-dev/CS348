




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodObjectivesDto {
    #[serde(rename = "baron")]
    pub baron: Box<crate::models::MatchV5PeriodObjectiveDto>,
    #[serde(rename = "champion")]
    pub champion: Box<crate::models::MatchV5PeriodObjectiveDto>,
    #[serde(rename = "dragon")]
    pub dragon: Box<crate::models::MatchV5PeriodObjectiveDto>,
    #[serde(rename = "inhibitor")]
    pub inhibitor: Box<crate::models::MatchV5PeriodObjectiveDto>,
    #[serde(rename = "riftHerald")]
    pub rift_herald: Box<crate::models::MatchV5PeriodObjectiveDto>,
    #[serde(rename = "tower")]
    pub tower: Box<crate::models::MatchV5PeriodObjectiveDto>,
}

impl MatchV5PeriodObjectivesDto {
    pub fn new(baron: crate::models::MatchV5PeriodObjectiveDto, champion: crate::models::MatchV5PeriodObjectiveDto, dragon: crate::models::MatchV5PeriodObjectiveDto, inhibitor: crate::models::MatchV5PeriodObjectiveDto, rift_herald: crate::models::MatchV5PeriodObjectiveDto, tower: crate::models::MatchV5PeriodObjectiveDto) -> MatchV5PeriodObjectivesDto {
        MatchV5PeriodObjectivesDto {
            baron: Box::new(baron),
            champion: Box::new(champion),
            dragon: Box::new(dragon),
            inhibitor: Box::new(inhibitor),
            rift_herald: Box::new(rift_herald),
            tower: Box::new(tower),
        }
    }
}


