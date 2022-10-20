#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LeagueExpV4PeriodLeagueEntryDto {
    #[serde(rename = "leagueId")]
    pub league_id: String,
    /// Player's summonerId (Encrypted)
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    #[serde(rename = "tier")]
    pub tier: String,
    /// The player's division within a tier.
    #[serde(rename = "rank")]
    pub rank: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    /// Winning team on Summoners Rift. First placement in Teamfight Tactics.
    #[serde(rename = "wins")]
    pub wins: i32,
    /// Losing team on Summoners Rift. Second through eighth placement in Teamfight Tactics.
    #[serde(rename = "losses")]
    pub losses: i32,
    #[serde(rename = "hotStreak")]
    pub hot_streak: bool,
    #[serde(rename = "veteran")]
    pub veteran: bool,
    #[serde(rename = "freshBlood")]
    pub fresh_blood: bool,
    #[serde(rename = "inactive")]
    pub inactive: bool,
    #[serde(rename = "miniSeries", skip_serializing_if = "Option::is_none")]
    pub mini_series: Option<Box<crate::models::LeagueExpV4PeriodMiniSeriesDto>>,
}

impl LeagueExpV4PeriodLeagueEntryDto {
    pub fn new(
        league_id: String,
        summoner_id: String,
        summoner_name: String,
        queue_type: String,
        tier: String,
        rank: String,
        league_points: i32,
        wins: i32,
        losses: i32,
        hot_streak: bool,
        veteran: bool,
        fresh_blood: bool,
        inactive: bool,
    ) -> LeagueExpV4PeriodLeagueEntryDto {
        LeagueExpV4PeriodLeagueEntryDto {
            league_id,
            summoner_id,
            summoner_name,
            queue_type,
            tier,
            rank,
            league_points,
            wins,
            losses,
            hot_streak,
            veteran,
            fresh_blood,
            inactive,
            mini_series: None,
        }
    }
}
