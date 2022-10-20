#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodMatchDto {
    #[serde(rename = "matchInfo")]
    pub match_info: Box<crate::models::ValMatchV1PeriodMatchInfoDto>,
    #[serde(rename = "players")]
    pub players: Vec<crate::models::ValMatchV1PeriodPlayerDto>,
    #[serde(rename = "coaches")]
    pub coaches: Vec<crate::models::ValMatchV1PeriodCoachDto>,
    #[serde(rename = "teams")]
    pub teams: Vec<crate::models::ValMatchV1PeriodTeamDto>,
    #[serde(rename = "roundResults")]
    pub round_results: Vec<crate::models::ValMatchV1PeriodRoundResultDto>,
}

impl ValMatchV1PeriodMatchDto {
    pub fn new(
        match_info: crate::models::ValMatchV1PeriodMatchInfoDto,
        players: Vec<crate::models::ValMatchV1PeriodPlayerDto>,
        coaches: Vec<crate::models::ValMatchV1PeriodCoachDto>,
        teams: Vec<crate::models::ValMatchV1PeriodTeamDto>,
        round_results: Vec<crate::models::ValMatchV1PeriodRoundResultDto>,
    ) -> ValMatchV1PeriodMatchDto {
        ValMatchV1PeriodMatchDto {
            match_info: Box::new(match_info),
            players,
            coaches,
            teams,
            round_results,
        }
    }
}
