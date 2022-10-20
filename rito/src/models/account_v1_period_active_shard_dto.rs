#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountV1PeriodActiveShardDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "game")]
    pub game: String,
    #[serde(rename = "activeShard")]
    pub active_shard: String,
}

impl AccountV1PeriodActiveShardDto {
    pub fn new(puuid: String, game: String, active_shard: String) -> AccountV1PeriodActiveShardDto {
        AccountV1PeriodActiveShardDto {
            puuid,
            game,
            active_shard,
        }
    }
}
