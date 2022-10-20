#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpectatorV4PeriodObserver {
    /// Key used to decrypt the spectator grid game data for playback
    #[serde(rename = "encryptionKey")]
    pub encryption_key: String,
}

impl SpectatorV4PeriodObserver {
    pub fn new(encryption_key: String) -> SpectatorV4PeriodObserver {
        SpectatorV4PeriodObserver { encryption_key }
    }
}
