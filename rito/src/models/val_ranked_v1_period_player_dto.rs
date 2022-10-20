#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValRankedV1PeriodPlayerDto {
    /// This field may be omitted if the player has been anonymized.
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    /// This field may be omitted if the player has been anonymized.
    #[serde(rename = "gameName", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    /// This field may be omitted if the player has been anonymized.
    #[serde(rename = "tagLine", skip_serializing_if = "Option::is_none")]
    pub tag_line: Option<String>,
    #[serde(rename = "leaderboardRank")]
    pub leaderboard_rank: i64,
    #[serde(rename = "rankedRating")]
    pub ranked_rating: i64,
    #[serde(rename = "numberOfWins")]
    pub number_of_wins: i64,
    #[serde(rename = "competitiveTier", skip_serializing_if = "Option::is_none")]
    pub competitive_tier: Option<i64>,
}

impl ValRankedV1PeriodPlayerDto {
    pub fn new(
        leaderboard_rank: i64,
        ranked_rating: i64,
        number_of_wins: i64,
    ) -> ValRankedV1PeriodPlayerDto {
        ValRankedV1PeriodPlayerDto {
            puuid: None,
            game_name: None,
            tag_line: None,
            leaderboard_rank,
            ranked_rating,
            number_of_wins,
            competitive_tier: None,
        }
    }
}
