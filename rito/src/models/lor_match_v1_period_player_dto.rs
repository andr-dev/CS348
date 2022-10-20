




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorMatchV1PeriodPlayerDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "deck_id")]
    pub deck_id: String,
    /// Code for the deck played. Refer to LOR documentation for details on deck codes.
    #[serde(rename = "deck_code")]
    pub deck_code: String,
    #[serde(rename = "factions")]
    pub factions: Vec<String>,
    #[serde(rename = "game_outcome")]
    pub game_outcome: String,
    /// The order in which the players took turns.
    #[serde(rename = "order_of_play")]
    pub order_of_play: i32,
}

impl LorMatchV1PeriodPlayerDto {
    pub fn new(puuid: String, deck_id: String, deck_code: String, factions: Vec<String>, game_outcome: String, order_of_play: i32) -> LorMatchV1PeriodPlayerDto {
        LorMatchV1PeriodPlayerDto {
            puuid,
            deck_id,
            deck_code,
            factions,
            game_outcome,
            order_of_play,
        }
    }
}


