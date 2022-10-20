#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodTeamDto {
    /// This is an arbitrary string. Red and Blue in bomb modes. The puuid of the player in deathmatch.
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "won")]
    pub won: bool,
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: i32,
    #[serde(rename = "roundsWon")]
    pub rounds_won: i32,
    /// Team points scored. Number of kills in deathmatch.
    #[serde(rename = "numPoints")]
    pub num_points: i32,
}

impl ValMatchV1PeriodTeamDto {
    pub fn new(
        team_id: String,
        won: bool,
        rounds_played: i32,
        rounds_won: i32,
        num_points: i32,
    ) -> ValMatchV1PeriodTeamDto {
        ValMatchV1PeriodTeamDto {
            team_id,
            won,
            rounds_played,
            rounds_won,
            num_points,
        }
    }
}
