




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolChallengesV1PeriodApexPlayerInfoDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "position")]
    pub position: i32,
}

impl LolChallengesV1PeriodApexPlayerInfoDto {
    pub fn new(puuid: String, value: f64, position: i32) -> LolChallengesV1PeriodApexPlayerInfoDto {
        LolChallengesV1PeriodApexPlayerInfoDto {
            puuid,
            value,
            position,
        }
    }
}


