




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValMatchV1PeriodPlayerDto {
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "characterId")]
    pub character_id: String,
    #[serde(rename = "stats")]
    pub stats: Box<crate::models::ValMatchV1PeriodPlayerStatsDto>,
    #[serde(rename = "competitiveTier")]
    pub competitive_tier: i32,
    #[serde(rename = "playerCard")]
    pub player_card: String,
    #[serde(rename = "playerTitle")]
    pub player_title: String,
}

impl ValMatchV1PeriodPlayerDto {
    pub fn new(puuid: String, game_name: String, tag_line: String, team_id: String, party_id: String, character_id: String, stats: crate::models::ValMatchV1PeriodPlayerStatsDto, competitive_tier: i32, player_card: String, player_title: String) -> ValMatchV1PeriodPlayerDto {
        ValMatchV1PeriodPlayerDto {
            puuid,
            game_name,
            tag_line,
            team_id,
            party_id,
            character_id,
            stats: Box::new(stats),
            competitive_tier,
            player_card,
            player_title,
        }
    }
}


