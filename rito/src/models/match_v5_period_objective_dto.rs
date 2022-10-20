




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodObjectiveDto {
    #[serde(rename = "first")]
    pub first: bool,
    #[serde(rename = "kills")]
    pub kills: i32,
}

impl MatchV5PeriodObjectiveDto {
    pub fn new(first: bool, kills: i32) -> MatchV5PeriodObjectiveDto {
        MatchV5PeriodObjectiveDto {
            first,
            kills,
        }
    }
}


