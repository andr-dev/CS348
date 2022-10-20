




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftMatchV1PeriodParticipantDto {
    #[serde(rename = "companion")]
    pub companion: Box<crate::models::TftMatchV1PeriodCompanionDto>,
    /// Gold left after participant was eliminated.
    #[serde(rename = "gold_left")]
    pub gold_left: i32,
    /// The round the participant was eliminated in. Note: If the player was eliminated in stage 2-1 their last_round would be 5.
    #[serde(rename = "last_round")]
    pub last_round: i32,
    /// Participant Little Legend level. Note: This is not the number of active units.
    #[serde(rename = "level")]
    pub level: i32,
    /// Participant placement upon elimination.
    #[serde(rename = "placement")]
    pub placement: i32,
    /// Number of players the participant eliminated.
    #[serde(rename = "players_eliminated")]
    pub players_eliminated: i32,
    #[serde(rename = "puuid")]
    pub puuid: String,
    /// The number of seconds before the participant was eliminated.
    #[serde(rename = "time_eliminated")]
    pub time_eliminated: f32,
    /// Damage the participant dealt to other players.
    #[serde(rename = "total_damage_to_players")]
    pub total_damage_to_players: i32,
    /// A complete list of traits for the participant's active units.
    #[serde(rename = "traits")]
    pub traits: Vec<crate::models::TftMatchV1PeriodTraitDto>,
    /// A list of active units for the participant.
    #[serde(rename = "units")]
    pub units: Vec<crate::models::TftMatchV1PeriodUnitDto>,
}

impl TftMatchV1PeriodParticipantDto {
    pub fn new(companion: crate::models::TftMatchV1PeriodCompanionDto, gold_left: i32, last_round: i32, level: i32, placement: i32, players_eliminated: i32, puuid: String, time_eliminated: f32, total_damage_to_players: i32, traits: Vec<crate::models::TftMatchV1PeriodTraitDto>, units: Vec<crate::models::TftMatchV1PeriodUnitDto>) -> TftMatchV1PeriodParticipantDto {
        TftMatchV1PeriodParticipantDto {
            companion: Box::new(companion),
            gold_left,
            last_round,
            level,
            placement,
            players_eliminated,
            puuid,
            time_eliminated,
            total_damage_to_players,
            traits,
            units,
        }
    }
}


