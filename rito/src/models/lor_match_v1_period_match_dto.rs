#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorMatchV1PeriodMatchDto {
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::LorMatchV1PeriodMetadataDto>,
    #[serde(rename = "info")]
    pub info: Box<crate::models::LorMatchV1PeriodInfoDto>,
}

impl LorMatchV1PeriodMatchDto {
    pub fn new(
        metadata: crate::models::LorMatchV1PeriodMetadataDto,
        info: crate::models::LorMatchV1PeriodInfoDto,
    ) -> LorMatchV1PeriodMatchDto {
        LorMatchV1PeriodMatchDto {
            metadata: Box::new(metadata),
            info: Box::new(info),
        }
    }
}
