




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LeagueExpV4PeriodMiniSeriesDto {
    #[serde(rename = "losses")]
    pub losses: i32,
    #[serde(rename = "progress")]
    pub progress: String,
    #[serde(rename = "target")]
    pub target: i32,
    #[serde(rename = "wins")]
    pub wins: i32,
}

impl LeagueExpV4PeriodMiniSeriesDto {
    pub fn new(losses: i32, progress: String, target: i32, wins: i32) -> LeagueExpV4PeriodMiniSeriesDto {
        LeagueExpV4PeriodMiniSeriesDto {
            losses,
            progress,
            target,
            wins,
        }
    }
}


