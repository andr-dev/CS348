




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodLocationDto {
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
}

impl ValMatchV1PeriodLocationDto {
    pub fn new(x: i32, y: i32) -> ValMatchV1PeriodLocationDto {
        ValMatchV1PeriodLocationDto {
            x,
            y,
        }
    }
}


