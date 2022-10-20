#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftLeagueV1PeriodTopRatedLadderEntryDto {
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    /// (Legal values:  ORANGE,  PURPLE,  BLUE,  GREEN,  GRAY)
    #[serde(rename = "ratedTier")]
    pub rated_tier: RatedTier,
    #[serde(rename = "ratedRating")]
    pub rated_rating: i32,
    /// First placement.
    #[serde(rename = "wins")]
    pub wins: i32,
    #[serde(rename = "previousUpdateLadderPosition")]
    pub previous_update_ladder_position: i32,
}

impl TftLeagueV1PeriodTopRatedLadderEntryDto {
    pub fn new(
        summoner_id: String,
        summoner_name: String,
        rated_tier: RatedTier,
        rated_rating: i32,
        wins: i32,
        previous_update_ladder_position: i32,
    ) -> TftLeagueV1PeriodTopRatedLadderEntryDto {
        TftLeagueV1PeriodTopRatedLadderEntryDto {
            summoner_id,
            summoner_name,
            rated_tier,
            rated_rating,
            wins,
            previous_update_ladder_position,
        }
    }
}

/// (Legal values:  ORANGE,  PURPLE,  BLUE,  GREEN,  GRAY)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatedTier {
    #[serde(rename = "ORANGE")]
    Orange,
    #[serde(rename = "PURPLE")]
    Purple,
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "GRAY")]
    Gray,
}

impl Default for RatedTier {
    fn default() -> RatedTier {
        Self::Orange
    }
}
