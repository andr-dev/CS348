




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodRecentMatchesDto {
    #[serde(rename = "currentTime")]
    pub current_time: i64,
    /// A list of recent match ids.
    #[serde(rename = "matchIds")]
    pub match_ids: Vec<String>,
}

impl ValMatchV1PeriodRecentMatchesDto {
    pub fn new(current_time: i64, match_ids: Vec<String>) -> ValMatchV1PeriodRecentMatchesDto {
        ValMatchV1PeriodRecentMatchesDto {
            current_time,
            match_ids,
        }
    }
}


