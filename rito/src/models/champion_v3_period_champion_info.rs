




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChampionV3PeriodChampionInfo {
    #[serde(rename = "maxNewPlayerLevel")]
    pub max_new_player_level: i32,
    #[serde(rename = "freeChampionIdsForNewPlayers")]
    pub free_champion_ids_for_new_players: Vec<i32>,
    #[serde(rename = "freeChampionIds")]
    pub free_champion_ids: Vec<i32>,
}

impl ChampionV3PeriodChampionInfo {
    pub fn new(max_new_player_level: i32, free_champion_ids_for_new_players: Vec<i32>, free_champion_ids: Vec<i32>) -> ChampionV3PeriodChampionInfo {
        ChampionV3PeriodChampionInfo {
            max_new_player_level,
            free_champion_ids_for_new_players,
            free_champion_ids,
        }
    }
}


