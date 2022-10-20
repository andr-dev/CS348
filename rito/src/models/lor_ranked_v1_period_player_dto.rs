#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorRankedV1PeriodPlayerDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rank")]
    pub rank: i32,
    /// League points.
    #[serde(rename = "lp")]
    pub lp: i32,
}

impl LorRankedV1PeriodPlayerDto {
    pub fn new(name: String, rank: i32, lp: i32) -> LorRankedV1PeriodPlayerDto {
        LorRankedV1PeriodPlayerDto { name, rank, lp }
    }
}
