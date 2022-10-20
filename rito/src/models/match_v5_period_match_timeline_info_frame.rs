#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoFrame {
    #[serde(rename = "events")]
    pub events: Vec<crate::models::MatchV5PeriodMatchTimelineInfoFrameEvent>,
    #[serde(rename = "participantFrames")]
    pub participant_frames:
        Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrames>,
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
}

impl MatchV5PeriodMatchTimelineInfoFrame {
    pub fn new(
        events: Vec<crate::models::MatchV5PeriodMatchTimelineInfoFrameEvent>,
        participant_frames: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrames,
        timestamp: i32,
    ) -> MatchV5PeriodMatchTimelineInfoFrame {
        MatchV5PeriodMatchTimelineInfoFrame {
            events,
            participant_frames: Box::new(participant_frames),
            timestamp,
        }
    }
}
