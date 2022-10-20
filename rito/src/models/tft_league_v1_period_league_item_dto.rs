#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftLeagueV1PeriodLeagueItemDto {
    #[serde(rename = "freshBlood")]
    pub fresh_blood: bool,
    /// First placement.
    #[serde(rename = "wins")]
    pub wins: i32,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "miniSeries", skip_serializing_if = "Option::is_none")]
    pub mini_series: Option<Box<crate::models::TftLeagueV1PeriodMiniSeriesDto>>,
    #[serde(rename = "inactive")]
    pub inactive: bool,
    #[serde(rename = "veteran")]
    pub veteran: bool,
    #[serde(rename = "hotStreak")]
    pub hot_streak: bool,
    #[serde(rename = "rank")]
    pub rank: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    /// Second through eighth placement.
    #[serde(rename = "losses")]
    pub losses: i32,
    /// Player's encrypted summonerId.
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
}

impl TftLeagueV1PeriodLeagueItemDto {
    pub fn new(
        fresh_blood: bool,
        wins: i32,
        summoner_name: String,
        inactive: bool,
        veteran: bool,
        hot_streak: bool,
        rank: String,
        league_points: i32,
        losses: i32,
        summoner_id: String,
    ) -> TftLeagueV1PeriodLeagueItemDto {
        TftLeagueV1PeriodLeagueItemDto {
            fresh_blood,
            wins,
            summoner_name,
            mini_series: None,
            inactive,
            veteran,
            hot_streak,
            rank,
            league_points,
            losses,
            summoner_id,
        }
    }
}
