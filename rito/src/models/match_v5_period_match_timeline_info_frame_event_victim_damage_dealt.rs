




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoFrameEventVictimDamageDealt {
    #[serde(rename = "basic")]
    pub basic: bool,
    #[serde(rename = "magicDamage")]
    pub magic_damage: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "participantId")]
    pub participant_id: i32,
    #[serde(rename = "physicalDamage")]
    pub physical_damage: i32,
    #[serde(rename = "spellName")]
    pub spell_name: String,
    #[serde(rename = "spellSlot")]
    pub spell_slot: i32,
    #[serde(rename = "trueDamage")]
    pub true_damage: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl MatchV5PeriodMatchTimelineInfoFrameEventVictimDamageDealt {
    pub fn new(basic: bool, magic_damage: i32, name: String, participant_id: i32, physical_damage: i32, spell_name: String, spell_slot: i32, true_damage: i32, r#type: String) -> MatchV5PeriodMatchTimelineInfoFrameEventVictimDamageDealt {
        MatchV5PeriodMatchTimelineInfoFrameEventVictimDamageDealt {
            basic,
            magic_damage,
            name,
            participant_id,
            physical_damage,
            spell_name,
            spell_slot,
            true_damage,
            r#type,
        }
    }
}


