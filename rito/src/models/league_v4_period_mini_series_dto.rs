




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LeagueV4PeriodMiniSeriesDto {
    #[serde(rename = "losses")]
    pub losses: i32,
    #[serde(rename = "progress")]
    pub progress: String,
    #[serde(rename = "target")]
    pub target: i32,
    #[serde(rename = "wins")]
    pub wins: i32,
}

impl LeagueV4PeriodMiniSeriesDto {
    pub fn new(losses: i32, progress: String, target: i32, wins: i32) -> LeagueV4PeriodMiniSeriesDto {
        LeagueV4PeriodMiniSeriesDto {
            losses,
            progress,
            target,
            wins,
        }
    }
}


