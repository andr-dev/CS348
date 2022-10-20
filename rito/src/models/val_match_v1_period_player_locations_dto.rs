#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodPlayerLocationsDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "viewRadians")]
    pub view_radians: f32,
    #[serde(rename = "location")]
    pub location: Box<crate::models::ValMatchV1PeriodLocationDto>,
}

impl ValMatchV1PeriodPlayerLocationsDto {
    pub fn new(
        puuid: String,
        view_radians: f32,
        location: crate::models::ValMatchV1PeriodLocationDto,
    ) -> ValMatchV1PeriodPlayerLocationsDto {
        ValMatchV1PeriodPlayerLocationsDto {
            puuid,
            view_radians,
            location: Box::new(location),
        }
    }
}
