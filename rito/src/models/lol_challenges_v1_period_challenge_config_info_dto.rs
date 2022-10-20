




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolChallengesV1PeriodChallengeConfigInfoDto {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "localizedNames")]
    pub localized_names: ::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>,
    /// DISABLED - not visible and not calculated, HIDDEN - not visible, but calculated, ENABLED - visible and calculated, ARCHIVED - visible, but not calculated
    #[serde(rename = "state")]
    pub state: State,
    /// LIFETIME - stats are incremented without reset, SEASON - stats are accumulated by season and reset at the beginning of new season
    #[serde(rename = "tracking", skip_serializing_if = "Option::is_none")]
    pub tracking: Option<Tracking>,
    #[serde(rename = "startTimestamp", skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    #[serde(rename = "endTimestamp", skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<i64>,
    #[serde(rename = "leaderboard")]
    pub leaderboard: bool,
    #[serde(rename = "thresholds")]
    pub thresholds: ::std::collections::HashMap<String, f64>,
}

impl LolChallengesV1PeriodChallengeConfigInfoDto {
    pub fn new(id: i64, localized_names: ::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>, state: State, leaderboard: bool, thresholds: ::std::collections::HashMap<String, f64>) -> LolChallengesV1PeriodChallengeConfigInfoDto {
        LolChallengesV1PeriodChallengeConfigInfoDto {
            id,
            localized_names,
            state,
            tracking: None,
            start_timestamp: None,
            end_timestamp: None,
            leaderboard,
            thresholds,
        }
    }
}

/// DISABLED - not visible and not calculated, HIDDEN - not visible, but calculated, ENABLED - visible and calculated, ARCHIVED - visible, but not calculated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "HIDDEN")]
    Hidden,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "ARCHIVED")]
    Archived,
}

impl Default for State {
    fn default() -> State {
        Self::Disabled
    }
}
/// LIFETIME - stats are incremented without reset, SEASON - stats are accumulated by season and reset at the beginning of new season
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Tracking {
    #[serde(rename = "LIFETIME")]
    Lifetime,
    #[serde(rename = "SEASON")]
    Season,
}

impl Default for Tracking {
    fn default() -> Tracking {
        Self::Lifetime
    }
}

