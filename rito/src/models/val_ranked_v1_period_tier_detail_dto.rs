




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValRankedV1PeriodTierDetailDto {
    #[serde(rename = "rankedRatingThreshold")]
    pub ranked_rating_threshold: i64,
    #[serde(rename = "startingPage")]
    pub starting_page: i64,
    #[serde(rename = "startingIndex")]
    pub starting_index: i64,
}

impl ValRankedV1PeriodTierDetailDto {
    pub fn new(ranked_rating_threshold: i64, starting_page: i64, starting_index: i64) -> ValRankedV1PeriodTierDetailDto {
        ValRankedV1PeriodTierDetailDto {
            ranked_rating_threshold,
            starting_page,
            starting_index,
        }
    }
}


