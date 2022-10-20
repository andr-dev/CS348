#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolChallengesV1PeriodPlayerClientPreferences {
    #[serde(rename = "bannerAccent", skip_serializing_if = "Option::is_none")]
    pub banner_accent: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "challengeIds", skip_serializing_if = "Option::is_none")]
    pub challenge_ids: Option<Vec<i64>>,
}

impl LolChallengesV1PeriodPlayerClientPreferences {
    pub fn new() -> LolChallengesV1PeriodPlayerClientPreferences {
        LolChallengesV1PeriodPlayerClientPreferences {
            banner_accent: None,
            title: None,
            challenge_ids: None,
        }
    }
}
