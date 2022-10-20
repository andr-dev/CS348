




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LeagueV4PeriodLeagueEntryDto {
    #[serde(rename = "leagueId", skip_serializing_if = "Option::is_none")]
    pub league_id: Option<String>,
    /// Player's encrypted summonerId.
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// The player's division within a tier.
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    /// Winning team on Summoners Rift.
    #[serde(rename = "wins")]
    pub wins: i32,
    /// Losing team on Summoners Rift.
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
    pub mini_series: Option<Box<crate::models::LeagueV4PeriodMiniSeriesDto>>,
}

impl LeagueV4PeriodLeagueEntryDto {
    pub fn new(summoner_id: String, summoner_name: String, queue_type: String, league_points: i32, wins: i32, losses: i32, hot_streak: bool, veteran: bool, fresh_blood: bool, inactive: bool) -> LeagueV4PeriodLeagueEntryDto {
        LeagueV4PeriodLeagueEntryDto {
            league_id: None,
            summoner_id,
            summoner_name,
            queue_type,
            tier: None,
            rank: None,
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


