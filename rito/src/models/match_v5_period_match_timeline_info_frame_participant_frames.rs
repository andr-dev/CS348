




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoFrameParticipantFrames {
    #[serde(rename = "1")]
    pub param_1: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "2")]
    pub param_2: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "3")]
    pub param_3: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "4")]
    pub param_4: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "5")]
    pub param_5: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "6")]
    pub param_6: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "7")]
    pub param_7: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "8")]
    pub param_8: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "9")]
    pub param_9: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
    #[serde(rename = "10")]
    pub param_10: Box<crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame>,
}

impl MatchV5PeriodMatchTimelineInfoFrameParticipantFrames {
    pub fn new(param_1: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_2: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_3: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_4: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_5: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_6: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_7: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_8: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_9: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame, param_10: crate::models::MatchV5PeriodMatchTimelineInfoFrameParticipantFrame) -> MatchV5PeriodMatchTimelineInfoFrameParticipantFrames {
        MatchV5PeriodMatchTimelineInfoFrameParticipantFrames {
            param_1: Box::new(param_1),
            param_2: Box::new(param_2),
            param_3: Box::new(param_3),
            param_4: Box::new(param_4),
            param_5: Box::new(param_5),
            param_6: Box::new(param_6),
            param_7: Box::new(param_7),
            param_8: Box::new(param_8),
            param_9: Box::new(param_9),
            param_10: Box::new(param_10),
        }
    }
}


