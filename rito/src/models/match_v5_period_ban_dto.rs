




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodBanDto {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "pickTurn")]
    pub pick_turn: i32,
}

impl MatchV5PeriodBanDto {
    pub fn new(champion_id: i32, pick_turn: i32) -> MatchV5PeriodBanDto {
        MatchV5PeriodBanDto {
            champion_id,
            pick_turn,
        }
    }
}


