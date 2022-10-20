/// TftSummonerV1PeriodSummonerDto : represents a summoner

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftSummonerV1PeriodSummonerDto {
    /// Encrypted account ID. Max length 56 characters.
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// ID of the summoner icon associated with the summoner.
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    /// Date summoner was last modified specified as epoch milliseconds. The following events will update this timestamp: summoner name change, summoner level change, or profile icon change.
    #[serde(rename = "revisionDate")]
    pub revision_date: i64,
    /// Summoner name.
    #[serde(rename = "name")]
    pub name: String,
    /// Encrypted summoner ID. Max length 63 characters.
    #[serde(rename = "id")]
    pub id: String,
    /// Encrypted PUUID. Exact length of 78 characters.
    #[serde(rename = "puuid")]
    pub puuid: String,
    /// Summoner level associated with the summoner.
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,
}

impl TftSummonerV1PeriodSummonerDto {
    /// represents a summoner
    pub fn new(
        account_id: String,
        profile_icon_id: i32,
        revision_date: i64,
        name: String,
        id: String,
        puuid: String,
        summoner_level: i64,
    ) -> TftSummonerV1PeriodSummonerDto {
        TftSummonerV1PeriodSummonerDto {
            account_id,
            profile_icon_id,
            revision_date,
            name,
            id,
            puuid,
            summoner_level,
        }
    }
}
