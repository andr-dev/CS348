#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolChallengesV1PeriodChallengeInfo {
    #[serde(rename = "challengeId")]
    pub challenge_id: i64,
    #[serde(rename = "percentile")]
    pub percentile: f64,
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "achievedTime", skip_serializing_if = "Option::is_none")]
    pub achieved_time: Option<i64>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(rename = "playersInLevel", skip_serializing_if = "Option::is_none")]
    pub players_in_level: Option<i64>,
}

impl LolChallengesV1PeriodChallengeInfo {
    pub fn new(
        challenge_id: i64,
        percentile: f64,
        level: String,
        value: f64,
    ) -> LolChallengesV1PeriodChallengeInfo {
        LolChallengesV1PeriodChallengeInfo {
            challenge_id,
            percentile,
            level,
            value,
            achieved_time: None,
            position: None,
            players_in_level: None,
        }
    }
}
