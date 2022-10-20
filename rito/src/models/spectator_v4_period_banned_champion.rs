




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodBannedChampion {
    /// The turn during which the champion was banned
    #[serde(rename = "pickTurn")]
    pub pick_turn: i32,
    /// The ID of the banned champion
    #[serde(rename = "championId")]
    pub champion_id: i64,
    /// The ID of the team that banned the champion
    #[serde(rename = "teamId")]
    pub team_id: i64,
}

impl SpectatorV4PeriodBannedChampion {
    pub fn new(pick_turn: i32, champion_id: i64, team_id: i64) -> SpectatorV4PeriodBannedChampion {
        SpectatorV4PeriodBannedChampion {
            pick_turn,
            champion_id,
            team_id,
        }
    }
}


