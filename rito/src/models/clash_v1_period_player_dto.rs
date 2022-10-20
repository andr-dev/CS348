




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClashV1PeriodPlayerDto {
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// (Legal values:  UNSELECTED,  FILL,  TOP,  JUNGLE,  MIDDLE,  BOTTOM,  UTILITY)
    #[serde(rename = "position")]
    pub position: Position,
    /// (Legal values:  CAPTAIN,  MEMBER)
    #[serde(rename = "role")]
    pub role: Role,
}

impl ClashV1PeriodPlayerDto {
    pub fn new(summoner_id: String, position: Position, role: Role) -> ClashV1PeriodPlayerDto {
        ClashV1PeriodPlayerDto {
            summoner_id,
            team_id: None,
            position,
            role,
        }
    }
}

/// (Legal values:  UNSELECTED,  FILL,  TOP,  JUNGLE,  MIDDLE,  BOTTOM,  UTILITY)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Position {
    #[serde(rename = "UNSELECTED")]
    Unselected,
    #[serde(rename = "FILL")]
    Fill,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "JUNGLE")]
    Jungle,
    #[serde(rename = "MIDDLE")]
    Middle,
    #[serde(rename = "BOTTOM")]
    Bottom,
    #[serde(rename = "UTILITY")]
    Utility,
}

impl Default for Position {
    fn default() -> Position {
        Self::Unselected
    }
}
/// (Legal values:  CAPTAIN,  MEMBER)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "CAPTAIN")]
    Captain,
    #[serde(rename = "MEMBER")]
    Member,
}

impl Default for Role {
    fn default() -> Role {
        Self::Captain
    }
}

