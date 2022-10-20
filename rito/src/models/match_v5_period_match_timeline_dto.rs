




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineDto {
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::MatchV5PeriodMetadataDto>,
    #[serde(rename = "info")]
    pub info: Box<crate::models::MatchV5PeriodMatchTimelineInfo>,
}

impl MatchV5PeriodMatchTimelineDto {
    pub fn new(metadata: crate::models::MatchV5PeriodMetadataDto, info: crate::models::MatchV5PeriodMatchTimelineInfo) -> MatchV5PeriodMatchTimelineDto {
        MatchV5PeriodMatchTimelineDto {
            metadata: Box::new(metadata),
            info: Box::new(info),
        }
    }
}


