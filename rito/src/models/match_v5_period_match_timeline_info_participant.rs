




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoParticipant {
    #[serde(rename = "participantId")]
    pub participant_id: i32,
    #[serde(rename = "puuid")]
    pub puuid: String,
}

impl MatchV5PeriodMatchTimelineInfoParticipant {
    pub fn new(participant_id: i32, puuid: String) -> MatchV5PeriodMatchTimelineInfoParticipant {
        MatchV5PeriodMatchTimelineInfoParticipant {
            participant_id,
            puuid,
        }
    }
}


