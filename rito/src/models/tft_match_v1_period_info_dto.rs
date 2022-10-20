#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftMatchV1PeriodInfoDto {
    /// Unix timestamp.
    #[serde(rename = "game_datetime")]
    pub game_datetime: i64,
    /// Game length in seconds.
    #[serde(rename = "game_length")]
    pub game_length: f32,
    /// Game variation key. Game variations documented in TFT static data.
    #[serde(rename = "game_variation", skip_serializing_if = "Option::is_none")]
    pub game_variation: Option<String>,
    /// Game client version.
    #[serde(rename = "game_version")]
    pub game_version: String,
    #[serde(rename = "participants")]
    pub participants: Vec<crate::models::TftMatchV1PeriodParticipantDto>,
    /// Please refer to the League of Legends documentation.
    #[serde(rename = "queue_id")]
    pub queue_id: i32,
    /// Teamfight Tactics set number.
    #[serde(rename = "tft_set_number")]
    pub tft_set_number: i32,
}

impl TftMatchV1PeriodInfoDto {
    pub fn new(
        game_datetime: i64,
        game_length: f32,
        game_version: String,
        participants: Vec<crate::models::TftMatchV1PeriodParticipantDto>,
        queue_id: i32,
        tft_set_number: i32,
    ) -> TftMatchV1PeriodInfoDto {
        TftMatchV1PeriodInfoDto {
            game_datetime,
            game_length,
            game_variation: None,
            game_version,
            participants,
            queue_id,
            tft_set_number,
        }
    }
}
