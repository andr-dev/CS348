




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LeagueV4PeriodLeagueListDto {
    #[serde(rename = "leagueId")]
    pub league_id: String,
    #[serde(rename = "entries")]
    pub entries: Vec<crate::models::LeagueV4PeriodLeagueItemDto>,
    #[serde(rename = "tier")]
    pub tier: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "queue")]
    pub queue: String,
}

impl LeagueV4PeriodLeagueListDto {
    pub fn new(league_id: String, entries: Vec<crate::models::LeagueV4PeriodLeagueItemDto>, tier: String, name: String, queue: String) -> LeagueV4PeriodLeagueListDto {
        LeagueV4PeriodLeagueListDto {
            league_id,
            entries,
            tier,
            name,
            queue,
        }
    }
}


