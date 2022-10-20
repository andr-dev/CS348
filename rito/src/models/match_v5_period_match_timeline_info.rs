#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfo {
    #[serde(rename = "frameInterval")]
    pub frame_interval: i32,
    #[serde(rename = "frames")]
    pub frames: Vec<crate::models::MatchV5PeriodMatchTimelineInfoFrame>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<crate::models::MatchV5PeriodMatchTimelineInfoParticipant>>,
}

impl MatchV5PeriodMatchTimelineInfo {
    pub fn new(
        frame_interval: i32,
        frames: Vec<crate::models::MatchV5PeriodMatchTimelineInfoFrame>,
    ) -> MatchV5PeriodMatchTimelineInfo {
        MatchV5PeriodMatchTimelineInfo {
            frame_interval,
            frames,
            game_id: None,
            participants: None,
        }
    }
}
