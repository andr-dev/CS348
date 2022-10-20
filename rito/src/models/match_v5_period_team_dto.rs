#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodTeamDto {
    #[serde(rename = "bans")]
    pub bans: Vec<crate::models::MatchV5PeriodBanDto>,
    #[serde(rename = "objectives")]
    pub objectives: Box<crate::models::MatchV5PeriodObjectivesDto>,
    #[serde(rename = "teamId")]
    pub team_id: i32,
    #[serde(rename = "win")]
    pub win: bool,
}

impl MatchV5PeriodTeamDto {
    pub fn new(
        bans: Vec<crate::models::MatchV5PeriodBanDto>,
        objectives: crate::models::MatchV5PeriodObjectivesDto,
        team_id: i32,
        win: bool,
    ) -> MatchV5PeriodTeamDto {
        MatchV5PeriodTeamDto {
            bans,
            objectives: Box::new(objectives),
            team_id,
            win,
        }
    }
}
