#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolChallengesV1PeriodPlayerInfoDto {
    #[serde(rename = "challenges")]
    pub challenges: Vec<crate::models::LolChallengesV1PeriodChallengeInfo>,
    #[serde(rename = "preferences")]
    pub preferences: Box<crate::models::LolChallengesV1PeriodPlayerClientPreferences>,
    #[serde(rename = "totalPoints")]
    pub total_points: Box<crate::models::LolChallengesV1PeriodChallengePoints>,
    #[serde(rename = "categoryPoints")]
    pub category_points:
        ::std::collections::HashMap<String, crate::models::LolChallengesV1PeriodChallengePoints>,
}

impl LolChallengesV1PeriodPlayerInfoDto {
    pub fn new(
        challenges: Vec<crate::models::LolChallengesV1PeriodChallengeInfo>,
        preferences: crate::models::LolChallengesV1PeriodPlayerClientPreferences,
        total_points: crate::models::LolChallengesV1PeriodChallengePoints,
        category_points: ::std::collections::HashMap<
            String,
            crate::models::LolChallengesV1PeriodChallengePoints,
        >,
    ) -> LolChallengesV1PeriodPlayerInfoDto {
        LolChallengesV1PeriodPlayerInfoDto {
            challenges,
            preferences: Box::new(preferences),
            total_points: Box::new(total_points),
            category_points,
        }
    }
}
