




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValContentV1PeriodContentDto {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "characters")]
    pub characters: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "maps")]
    pub maps: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "chromas")]
    pub chromas: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "skins")]
    pub skins: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "skinLevels")]
    pub skin_levels: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "equips")]
    pub equips: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "gameModes")]
    pub game_modes: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "sprays")]
    pub sprays: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "sprayLevels")]
    pub spray_levels: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "charms")]
    pub charms: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "charmLevels")]
    pub charm_levels: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "playerCards")]
    pub player_cards: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "playerTitles")]
    pub player_titles: Vec<crate::models::ValContentV1PeriodContentItemDto>,
    #[serde(rename = "acts")]
    pub acts: Vec<crate::models::ValContentV1PeriodActDto>,
    #[serde(rename = "ceremonies", skip_serializing_if = "Option::is_none")]
    pub ceremonies: Option<Vec<crate::models::ValContentV1PeriodContentItemDto>>,
}

impl ValContentV1PeriodContentDto {
    pub fn new(version: String, characters: Vec<crate::models::ValContentV1PeriodContentItemDto>, maps: Vec<crate::models::ValContentV1PeriodContentItemDto>, chromas: Vec<crate::models::ValContentV1PeriodContentItemDto>, skins: Vec<crate::models::ValContentV1PeriodContentItemDto>, skin_levels: Vec<crate::models::ValContentV1PeriodContentItemDto>, equips: Vec<crate::models::ValContentV1PeriodContentItemDto>, game_modes: Vec<crate::models::ValContentV1PeriodContentItemDto>, sprays: Vec<crate::models::ValContentV1PeriodContentItemDto>, spray_levels: Vec<crate::models::ValContentV1PeriodContentItemDto>, charms: Vec<crate::models::ValContentV1PeriodContentItemDto>, charm_levels: Vec<crate::models::ValContentV1PeriodContentItemDto>, player_cards: Vec<crate::models::ValContentV1PeriodContentItemDto>, player_titles: Vec<crate::models::ValContentV1PeriodContentItemDto>, acts: Vec<crate::models::ValContentV1PeriodActDto>) -> ValContentV1PeriodContentDto {
        ValContentV1PeriodContentDto {
            version,
            characters,
            maps,
            chromas,
            skins,
            skin_levels,
            equips,
            game_modes,
            sprays,
            spray_levels,
            charms,
            charm_levels,
            player_cards,
            player_titles,
            acts,
            ceremonies: None,
        }
    }
}


