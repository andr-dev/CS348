#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodPerkStyleSelectionDto {
    #[serde(rename = "perk")]
    pub perk: i32,
    #[serde(rename = "var1")]
    pub var1: i32,
    #[serde(rename = "var2")]
    pub var2: i32,
    #[serde(rename = "var3")]
    pub var3: i32,
}

impl MatchV5PeriodPerkStyleSelectionDto {
    pub fn new(perk: i32, var1: i32, var2: i32, var3: i32) -> MatchV5PeriodPerkStyleSelectionDto {
        MatchV5PeriodPerkStyleSelectionDto {
            perk,
            var1,
            var2,
            var3,
        }
    }
}
