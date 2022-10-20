




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftLeagueV1PeriodLeagueEntryDto {
    /// Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "leagueId", skip_serializing_if = "Option::is_none")]
    pub league_id: Option<String>,
    /// Player's encrypted summonerId.
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    /// Only included for the RANKED_TFT_TURBO queueType.              (Legal values:  ORANGE,  PURPLE,  BLUE,  GREEN,  GRAY)
    #[serde(rename = "ratedTier", skip_serializing_if = "Option::is_none")]
    pub rated_tier: Option<RatedTier>,
    /// Only included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "ratedRating", skip_serializing_if = "Option::is_none")]
    pub rated_rating: Option<i32>,
    /// Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// A player's division within a tier. Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    /// Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "leaguePoints", skip_serializing_if = "Option::is_none")]
    pub league_points: Option<i32>,
    /// First placement.
    #[serde(rename = "wins")]
    pub wins: i32,
    /// Second through eighth placement.
    #[serde(rename = "losses")]
    pub losses: i32,
    /// Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "hotStreak", skip_serializing_if = "Option::is_none")]
    pub hot_streak: Option<bool>,
    /// Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "veteran", skip_serializing_if = "Option::is_none")]
    pub veteran: Option<bool>,
    /// Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "freshBlood", skip_serializing_if = "Option::is_none")]
    pub fresh_blood: Option<bool>,
    /// Not included for the RANKED_TFT_TURBO queueType.
    #[serde(rename = "inactive", skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    #[serde(rename = "miniSeries", skip_serializing_if = "Option::is_none")]
    pub mini_series: Option<Box<crate::models::TftLeagueV1PeriodMiniSeriesDto>>,
}

impl TftLeagueV1PeriodLeagueEntryDto {
    pub fn new(summoner_id: String, summoner_name: String, queue_type: String, wins: i32, losses: i32) -> TftLeagueV1PeriodLeagueEntryDto {
        TftLeagueV1PeriodLeagueEntryDto {
            league_id: None,
            summoner_id,
            summoner_name,
            queue_type,
            rated_tier: None,
            rated_rating: None,
            tier: None,
            rank: None,
            league_points: None,
            wins,
            losses,
            hot_streak: None,
            veteran: None,
            fresh_blood: None,
            inactive: None,
            mini_series: None,
        }
    }
}

/// Only included for the RANKED_TFT_TURBO queueType.              (Legal values:  ORANGE,  PURPLE,  BLUE,  GREEN,  GRAY)
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

