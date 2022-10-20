#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchDto {
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::MatchV5PeriodMetadataDto>,
    #[serde(rename = "info")]
    pub info: Box<crate::models::MatchV5PeriodInfoDto>,
}

impl MatchV5PeriodMatchDto {
    pub fn new(
        metadata: crate::models::MatchV5PeriodMetadataDto,
        info: crate::models::MatchV5PeriodInfoDto,
    ) -> MatchV5PeriodMatchDto {
        MatchV5PeriodMatchDto {
            metadata: Box::new(metadata),
            info: Box::new(info),
        }
    }
}
