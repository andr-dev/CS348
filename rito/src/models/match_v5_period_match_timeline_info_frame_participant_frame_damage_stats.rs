#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoFrameParticipantFrameDamageStats {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i32,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i32,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i32,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i32,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i32,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i32,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i32,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i32,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i32,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i32,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i32,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i32,
}

impl MatchV5PeriodMatchTimelineInfoFrameParticipantFrameDamageStats {
    pub fn new(
        magic_damage_done: i32,
        magic_damage_done_to_champions: i32,
        magic_damage_taken: i32,
        physical_damage_done: i32,
        physical_damage_done_to_champions: i32,
        physical_damage_taken: i32,
        total_damage_done: i32,
        total_damage_done_to_champions: i32,
        total_damage_taken: i32,
        true_damage_done: i32,
        true_damage_done_to_champions: i32,
        true_damage_taken: i32,
    ) -> MatchV5PeriodMatchTimelineInfoFrameParticipantFrameDamageStats {
        MatchV5PeriodMatchTimelineInfoFrameParticipantFrameDamageStats {
            magic_damage_done,
            magic_damage_done_to_champions,
            magic_damage_taken,
            physical_damage_done,
            physical_damage_done_to_champions,
            physical_damage_taken,
            total_damage_done,
            total_damage_done_to_champions,
            total_damage_taken,
            true_damage_done,
            true_damage_done_to_champions,
            true_damage_taken,
        }
    }
}
