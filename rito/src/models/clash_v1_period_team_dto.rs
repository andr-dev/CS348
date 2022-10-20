




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClashV1PeriodTeamDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "iconId")]
    pub icon_id: i32,
    #[serde(rename = "tier")]
    pub tier: i32,
    /// Summoner ID of the team captain.
    #[serde(rename = "captain")]
    pub captain: String,
    #[serde(rename = "abbreviation")]
    pub abbreviation: String,
    /// Team members.
    #[serde(rename = "players")]
    pub players: Vec<crate::models::ClashV1PeriodPlayerDto>,
}

impl ClashV1PeriodTeamDto {
    pub fn new(id: String, tournament_id: i32, name: String, icon_id: i32, tier: i32, captain: String, abbreviation: String, players: Vec<crate::models::ClashV1PeriodPlayerDto>) -> ClashV1PeriodTeamDto {
        ClashV1PeriodTeamDto {
            id,
            tournament_id,
            name,
            icon_id,
            tier,
            captain,
            abbreviation,
            players,
        }
    }
}


