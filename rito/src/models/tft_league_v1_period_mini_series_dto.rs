#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftLeagueV1PeriodMiniSeriesDto {
    #[serde(rename = "losses")]
    pub losses: i32,
    #[serde(rename = "progress")]
    pub progress: String,
    #[serde(rename = "target")]
    pub target: i32,
    #[serde(rename = "wins")]
    pub wins: i32,
}

impl TftLeagueV1PeriodMiniSeriesDto {
    pub fn new(
        losses: i32,
        progress: String,
        target: i32,
        wins: i32,
    ) -> TftLeagueV1PeriodMiniSeriesDto {
        TftLeagueV1PeriodMiniSeriesDto {
            losses,
            progress,
            target,
            wins,
        }
    }
}
