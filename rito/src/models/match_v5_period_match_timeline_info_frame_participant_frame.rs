#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoFrameParticipantFrame {
    #[serde(rename = "championStats")]
    pub champion_stats:
        Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrameChampionStats>,
    #[serde(rename = "currentGold")]
    pub current_gold: i32,
    #[serde(rename = "damageStats")]
    pub damage_stats:
        Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrameDamageStats>,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i32,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i32,
    #[serde(rename = "participantId")]
    pub participant_id: i32,
    #[serde(rename = "position")]
    pub position: Box<crate::models::MatchV5PeriodMatchTimelinePosition>,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i32,
    #[serde(rename = "totalGold")]
    pub total_gold: i32,
    #[serde(rename = "xp")]
    pub xp: i32,
}

impl MatchV5PeriodMatchTimelineInfoFrameParticipantFrame {
    pub fn new(
        champion_stats: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrameChampionStats,
        current_gold: i32,
        damage_stats: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrameDamageStats,
        gold_per_second: i32,
        jungle_minions_killed: i32,
        level: i32,
        minions_killed: i32,
        participant_id: i32,
        position: crate::models::MatchV5PeriodMatchTimelinePosition,
        time_enemy_spent_controlled: i32,
        total_gold: i32,
        xp: i32,
    ) -> MatchV5PeriodMatchTimelineInfoFrameParticipantFrame {
        MatchV5PeriodMatchTimelineInfoFrameParticipantFrame {
            champion_stats: Box::new(champion_stats),
            current_gold,
            damage_stats: Box::new(damage_stats),
            gold_per_second,
            jungle_minions_killed,
            level,
            minions_killed,
            participant_id,
            position: Box::new(position),
            time_enemy_spent_controlled,
            total_gold,
            xp,
        }
    }
}
