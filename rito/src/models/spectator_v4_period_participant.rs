




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodParticipant {
    /// Flag indicating whether or not this participant is a bot
    #[serde(rename = "bot")]
    pub bot: bool,
    /// The ID of the second summoner spell used by this participant
    #[serde(rename = "spell2Id")]
    pub spell2_id: i64,
    /// The ID of the profile icon used by this participant
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i64,
    /// The summoner name of this participant
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    /// The ID of the champion played by this participant
    #[serde(rename = "championId")]
    pub champion_id: i64,
    /// The team ID of this participant, indicating the participant's team
    #[serde(rename = "teamId")]
    pub team_id: i64,
    /// The ID of the first summoner spell used by this participant
    #[serde(rename = "spell1Id")]
    pub spell1_id: i64,
}

impl SpectatorV4PeriodParticipant {
    pub fn new(bot: bool, spell2_id: i64, profile_icon_id: i64, summoner_name: String, champion_id: i64, team_id: i64, spell1_id: i64) -> SpectatorV4PeriodParticipant {
        SpectatorV4PeriodParticipant {
            bot,
            spell2_id,
            profile_icon_id,
            summoner_name,
            champion_id,
            team_id,
            spell1_id,
        }
    }
}


