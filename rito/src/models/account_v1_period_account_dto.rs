




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountV1PeriodAccountDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    /// This field may be excluded from the response if the account doesn't have a gameName.
    #[serde(rename = "gameName", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    /// This field may be excluded from the response if the account doesn't have a tagLine.
    #[serde(rename = "tagLine", skip_serializing_if = "Option::is_none")]
    pub tag_line: Option<String>,
}

impl AccountV1PeriodAccountDto {
    pub fn new(puuid: String) -> AccountV1PeriodAccountDto {
        AccountV1PeriodAccountDto {
            puuid,
            game_name: None,
            tag_line: None,
        }
    }
}


