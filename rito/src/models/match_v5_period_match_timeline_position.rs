




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelinePosition {
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
}

impl MatchV5PeriodMatchTimelinePosition {
    pub fn new(x: i32, y: i32) -> MatchV5PeriodMatchTimelinePosition {
        MatchV5PeriodMatchTimelinePosition {
            x,
            y,
        }
    }
}


