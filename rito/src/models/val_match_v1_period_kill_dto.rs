




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodKillDto {
    #[serde(rename = "timeSinceGameStartMillis")]
    pub time_since_game_start_millis: i32,
    #[serde(rename = "timeSinceRoundStartMillis")]
    pub time_since_round_start_millis: i32,
    /// PUUID
    #[serde(rename = "killer")]
    pub killer: String,
    /// PUUID
    #[serde(rename = "victim")]
    pub victim: String,
    #[serde(rename = "victimLocation")]
    pub victim_location: Box<crate::models::ValMatchV1PeriodLocationDto>,
    /// List of PUUIDs
    #[serde(rename = "assistants")]
    pub assistants: Vec<String>,
    #[serde(rename = "playerLocations")]
    pub player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>,
    #[serde(rename = "finishingDamage")]
    pub finishing_damage: Box<crate::models::ValMatchV1PeriodFinishingDamageDto>,
}

impl ValMatchV1PeriodKillDto {
    pub fn new(time_since_game_start_millis: i32, time_since_round_start_millis: i32, killer: String, victim: String, victim_location: crate::models::ValMatchV1PeriodLocationDto, assistants: Vec<String>, player_locations: Vec<crate::models::ValMatchV1PeriodPlayerLocationsDto>, finishing_damage: crate::models::ValMatchV1PeriodFinishingDamageDto) -> ValMatchV1PeriodKillDto {
        ValMatchV1PeriodKillDto {
            time_since_game_start_millis,
            time_since_round_start_millis,
            killer,
            victim,
            victim_location: Box::new(victim_location),
            assistants,
            player_locations,
            finishing_damage: Box::new(finishing_damage),
        }
    }
}


