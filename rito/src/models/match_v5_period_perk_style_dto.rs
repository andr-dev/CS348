#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodPerkStyleDto {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "selections")]
    pub selections: Vec<crate::models::MatchV5PeriodPerkStyleSelectionDto>,
    #[serde(rename = "style")]
    pub style: i32,
}

impl MatchV5PeriodPerkStyleDto {
    pub fn new(
        description: String,
        selections: Vec<crate::models::MatchV5PeriodPerkStyleSelectionDto>,
        style: i32,
    ) -> MatchV5PeriodPerkStyleDto {
        MatchV5PeriodPerkStyleDto {
            description,
            selections,
            style,
        }
    }
}
