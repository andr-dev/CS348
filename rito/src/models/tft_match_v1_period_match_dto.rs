#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftMatchV1PeriodMatchDto {
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::TftMatchV1PeriodMetadataDto>,
    #[serde(rename = "info")]
    pub info: Box<crate::models::TftMatchV1PeriodInfoDto>,
}

impl TftMatchV1PeriodMatchDto {
    pub fn new(
        metadata: crate::models::TftMatchV1PeriodMetadataDto,
        info: crate::models::TftMatchV1PeriodInfoDto,
    ) -> TftMatchV1PeriodMatchDto {
        TftMatchV1PeriodMatchDto {
            metadata: Box::new(metadata),
            info: Box::new(info),
        }
    }
}
