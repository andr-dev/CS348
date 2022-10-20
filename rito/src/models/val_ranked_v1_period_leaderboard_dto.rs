#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValRankedV1PeriodLeaderboardDto {
    /// The shard for the given leaderboard.
    #[serde(rename = "shard")]
    pub shard: String,
    /// The act id for the given leaderboard. Act ids can be found using the val-content API.
    #[serde(rename = "actId")]
    pub act_id: String,
    /// The total number of players in the leaderboard.
    #[serde(rename = "totalPlayers")]
    pub total_players: i64,
    #[serde(rename = "players")]
    pub players: Vec<crate::models::ValRankedV1PeriodPlayerDto>,
    #[serde(
        rename = "immortalStartingPage",
        skip_serializing_if = "Option::is_none"
    )]
    pub immortal_starting_page: Option<i64>,
    #[serde(
        rename = "immortalStartingIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub immortal_starting_index: Option<i64>,
    #[serde(rename = "topTierRRThreshold", skip_serializing_if = "Option::is_none")]
    pub top_tier_rr_threshold: Option<i64>,
    #[serde(rename = "tierDetails", skip_serializing_if = "Option::is_none")]
    pub tier_details:
        Option<::std::collections::HashMap<String, crate::models::ValRankedV1PeriodTierDetailDto>>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl ValRankedV1PeriodLeaderboardDto {
    pub fn new(
        shard: String,
        act_id: String,
        total_players: i64,
        players: Vec<crate::models::ValRankedV1PeriodPlayerDto>,
    ) -> ValRankedV1PeriodLeaderboardDto {
        ValRankedV1PeriodLeaderboardDto {
            shard,
            act_id,
            total_players,
            players,
            immortal_starting_page: None,
            immortal_starting_index: None,
            top_tier_rr_threshold: None,
            tier_details: None,
            start_index: None,
            query: None,
        }
    }
}
