#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolChallengesV1PeriodChallengePoints {
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "current")]
    pub current: i64,
    #[serde(rename = "max")]
    pub max: i64,
    #[serde(rename = "percentile", skip_serializing_if = "Option::is_none")]
    pub percentile: Option<f64>,
}

impl LolChallengesV1PeriodChallengePoints {
    pub fn new(level: String, current: i64, max: i64) -> LolChallengesV1PeriodChallengePoints {
        LolChallengesV1PeriodChallengePoints {
            level,
            current,
            max,
            percentile: None,
        }
    }
}
