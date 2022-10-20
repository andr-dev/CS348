#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoFrameEvent {
    #[serde(rename = "realTimestamp", skip_serializing_if = "Option::is_none")]
    pub real_timestamp: Option<i64>,
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Timeline event type. (Known legal values: ASCENDED_EVENT, BUILDING_KILL, CAPTURE_POINT, CHAMPION_KILL, CHAMPION_SPECIAL_KILL, CHAMPION_TRANSFORM, DRAGON_SOUL_GIVEN, ELITE_MONSTER_KILL, GAME_END, ITEM_DESTROYED, ITEM_PURCHASED, ITEM_SOLD, ITEM_UNDO, LEVEL_UP, PAUSE_END, PAUSE_START, SKILL_LEVEL_UP, TURRET_PLATE_DESTROYED, WARD_KILL, WARD_PLACED)
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<i32>,
    #[serde(rename = "levelUpType", skip_serializing_if = "Option::is_none")]
    pub level_up_type: Option<String>,
    #[serde(rename = "skillSlot", skip_serializing_if = "Option::is_none")]
    pub skill_slot: Option<i32>,
    #[serde(rename = "creatorId", skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i32>,
    #[serde(rename = "wardType", skip_serializing_if = "Option::is_none")]
    pub ward_type: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(
        rename = "assistingParticipantIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub assisting_participant_ids: Option<Vec<i32>>,
    #[serde(rename = "bounty", skip_serializing_if = "Option::is_none")]
    pub bounty: Option<i32>,
    #[serde(rename = "killStreakLength", skip_serializing_if = "Option::is_none")]
    pub kill_streak_length: Option<i32>,
    #[serde(rename = "killerId", skip_serializing_if = "Option::is_none")]
    pub killer_id: Option<i32>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::MatchV5PeriodMatchTimelinePosition>>,
    #[serde(rename = "victimDamageDealt", skip_serializing_if = "Option::is_none")]
    pub victim_damage_dealt:
        Option<Vec<crate::models::MatchV5PeriodMatchTimelineInfoFrameEventVictimDamageDealt>>,
    #[serde(
        rename = "victimDamageReceived",
        skip_serializing_if = "Option::is_none"
    )]
    pub victim_damage_received:
        Option<Vec<crate::models::MatchV5PeriodMatchTimelineInfoFrameEventVictimDamageDealt>>,
    #[serde(rename = "victimId", skip_serializing_if = "Option::is_none")]
    pub victim_id: Option<i32>,
    #[serde(rename = "killType", skip_serializing_if = "Option::is_none")]
    pub kill_type: Option<String>,
    #[serde(rename = "laneType", skip_serializing_if = "Option::is_none")]
    pub lane_type: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    #[serde(rename = "multiKillLength", skip_serializing_if = "Option::is_none")]
    pub multi_kill_length: Option<i32>,
    #[serde(rename = "killerTeamId", skip_serializing_if = "Option::is_none")]
    pub killer_team_id: Option<i32>,
    #[serde(rename = "monsterType", skip_serializing_if = "Option::is_none")]
    pub monster_type: Option<String>,
    #[serde(rename = "monsterSubType", skip_serializing_if = "Option::is_none")]
    pub monster_sub_type: Option<String>,
    #[serde(rename = "buildingType", skip_serializing_if = "Option::is_none")]
    pub building_type: Option<String>,
    #[serde(rename = "towerType", skip_serializing_if = "Option::is_none")]
    pub tower_type: Option<String>,
    #[serde(rename = "afterId", skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i32>,
    #[serde(rename = "beforeId", skip_serializing_if = "Option::is_none")]
    pub before_id: Option<i32>,
    #[serde(rename = "goldGain", skip_serializing_if = "Option::is_none")]
    pub gold_gain: Option<i32>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "winningTeam", skip_serializing_if = "Option::is_none")]
    pub winning_team: Option<i32>,
    #[serde(rename = "transformType", skip_serializing_if = "Option::is_none")]
    pub transform_type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "shutdownBounty", skip_serializing_if = "Option::is_none")]
    pub shutdown_bounty: Option<i32>,
    #[serde(rename = "actualStartTime", skip_serializing_if = "Option::is_none")]
    pub actual_start_time: Option<i64>,
}

impl MatchV5PeriodMatchTimelineInfoFrameEvent {
    pub fn new(timestamp: i32, r#type: RHashType) -> MatchV5PeriodMatchTimelineInfoFrameEvent {
        MatchV5PeriodMatchTimelineInfoFrameEvent {
            real_timestamp: None,
            timestamp,
            r#type,
            item_id: None,
            participant_id: None,
            level_up_type: None,
            skill_slot: None,
            creator_id: None,
            ward_type: None,
            level: None,
            assisting_participant_ids: None,
            bounty: None,
            kill_streak_length: None,
            killer_id: None,
            position: None,
            victim_damage_dealt: None,
            victim_damage_received: None,
            victim_id: None,
            kill_type: None,
            lane_type: None,
            team_id: None,
            multi_kill_length: None,
            killer_team_id: None,
            monster_type: None,
            monster_sub_type: None,
            building_type: None,
            tower_type: None,
            after_id: None,
            before_id: None,
            gold_gain: None,
            game_id: None,
            winning_team: None,
            transform_type: None,
            name: None,
            shutdown_bounty: None,
            actual_start_time: None,
        }
    }
}

/// Timeline event type. (Known legal values: ASCENDED_EVENT, BUILDING_KILL, CAPTURE_POINT, CHAMPION_KILL, CHAMPION_SPECIAL_KILL, CHAMPION_TRANSFORM, DRAGON_SOUL_GIVEN, ELITE_MONSTER_KILL, GAME_END, ITEM_DESTROYED, ITEM_PURCHASED, ITEM_SOLD, ITEM_UNDO, LEVEL_UP, PAUSE_END, PAUSE_START, SKILL_LEVEL_UP, TURRET_PLATE_DESTROYED, WARD_KILL, WARD_PLACED)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "ASCENDED_EVENT")]
    AscendedEvent,
    #[serde(rename = "BUILDING_KILL")]
    BuildingKill,
    #[serde(rename = "CAPTURE_POINT")]
    CapturePoint,
    #[serde(rename = "CHAMPION_KILL")]
    ChampionKill,
    #[serde(rename = "CHAMPION_SPECIAL_KILL")]
    ChampionSpecialKill,
    #[serde(rename = "CHAMPION_TRANSFORM")]
    ChampionTransform,
    #[serde(rename = "DRAGON_SOUL_GIVEN")]
    DragonSoulGiven,
    #[serde(rename = "ELITE_MONSTER_KILL")]
    EliteMonsterKill,
    #[serde(rename = "GAME_END")]
    GameEnd,
    #[serde(rename = "ITEM_DESTROYED")]
    ItemDestroyed,
    #[serde(rename = "ITEM_PURCHASED")]
    ItemPurchased,
    #[serde(rename = "ITEM_SOLD")]
    ItemSold,
    #[serde(rename = "ITEM_UNDO")]
    ItemUndo,
    #[serde(rename = "LEVEL_UP")]
    LevelUp,
    #[serde(rename = "PAUSE_END")]
    PauseEnd,
    #[serde(rename = "PAUSE_START")]
    PauseStart,
    #[serde(rename = "SKILL_LEVEL_UP")]
    SkillLevelUp,
    #[serde(rename = "TURRET_PLATE_DESTROYED")]
    TurretPlateDestroyed,
    #[serde(rename = "WARD_KILL")]
    WardKill,
    #[serde(rename = "WARD_PLACED")]
    WardPlaced,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::AscendedEvent
    }
}
