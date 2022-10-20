#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodFeaturedGames {
    /// The list of featured games
    #[serde(rename = "gameList")]
    pub game_list: Vec<crate::models::SpectatorV4PeriodFeaturedGameInfo>,
    /// The suggested interval to wait before requesting FeaturedGames again
    #[serde(rename = "clientRefreshInterval")]
    pub client_refresh_interval: i64,
}

impl SpectatorV4PeriodFeaturedGames {
    pub fn new(
        game_list: Vec<crate::models::SpectatorV4PeriodFeaturedGameInfo>,
        client_refresh_interval: i64,
    ) -> SpectatorV4PeriodFeaturedGames {
        SpectatorV4PeriodFeaturedGames {
            game_list,
            client_refresh_interval,
        }
    }
}
