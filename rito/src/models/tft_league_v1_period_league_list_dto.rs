




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftLeagueV1PeriodLeagueListDto {
    #[serde(rename = "leagueId")]
    pub league_id: String,
    #[serde(rename = "entries")]
    pub entries: Vec<crate::models::TftLeagueV1PeriodLeagueItemDto>,
    #[serde(rename = "tier")]
    pub tier: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "queue")]
    pub queue: String,
}

impl TftLeagueV1PeriodLeagueListDto {
    pub fn new(league_id: String, entries: Vec<crate::models::TftLeagueV1PeriodLeagueItemDto>, tier: String, name: String, queue: String) -> TftLeagueV1PeriodLeagueListDto {
        TftLeagueV1PeriodLeagueListDto {
            league_id,
            entries,
            tier,
            name,
            queue,
        }
    }
}


