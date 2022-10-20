




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodCurrentGameParticipant {
    /// The ID of the champion played by this participant
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "perks", skip_serializing_if = "Option::is_none")]
    pub perks: Option<Box<crate::models::SpectatorV4PeriodPerks>>,
    /// The ID of the profile icon used by this participant
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i64,
    /// Flag indicating whether or not this participant is a bot
    #[serde(rename = "bot")]
    pub bot: bool,
    /// The team ID of this participant, indicating the participant's team
    #[serde(rename = "teamId")]
    pub team_id: i64,
    /// The summoner name of this participant
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    /// The encrypted summoner ID of this participant
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    /// The ID of the first summoner spell used by this participant
    #[serde(rename = "spell1Id")]
    pub spell1_id: i64,
    /// The ID of the second summoner spell used by this participant
    #[serde(rename = "spell2Id")]
    pub spell2_id: i64,
    /// List of Game Customizations
    #[serde(rename = "gameCustomizationObjects")]
    pub game_customization_objects: Vec<crate::models::SpectatorV4PeriodGameCustomizationObject>,
}

impl SpectatorV4PeriodCurrentGameParticipant {
    pub fn new(champion_id: i64, profile_icon_id: i64, bot: bool, team_id: i64, summoner_name: String, summoner_id: String, spell1_id: i64, spell2_id: i64, game_customization_objects: Vec<crate::models::SpectatorV4PeriodGameCustomizationObject>) -> SpectatorV4PeriodCurrentGameParticipant {
        SpectatorV4PeriodCurrentGameParticipant {
            champion_id,
            perks: None,
            profile_icon_id,
            bot,
            team_id,
            summoner_name,
            summoner_id,
            spell1_id,
            spell2_id,
            game_customization_objects,
        }
    }
}


