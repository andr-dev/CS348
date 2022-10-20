




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorRankedV1PeriodLeaderboardDto {
    /// A list of players in Master tier.
    #[serde(rename = "players")]
    pub players: Vec<crate::models::LorRankedV1PeriodPlayerDto>,
}

impl LorRankedV1PeriodLeaderboardDto {
    pub fn new(players: Vec<crate::models::LorRankedV1PeriodPlayerDto>) -> LorRankedV1PeriodLeaderboardDto {
        LorRankedV1PeriodLeaderboardDto {
            players,
        }
    }
}


