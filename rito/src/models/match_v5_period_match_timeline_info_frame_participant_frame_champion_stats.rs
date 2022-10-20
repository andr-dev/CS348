




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMatchTimelineInfoFrameParticipantFrameChampionStats {
    #[serde(rename = "abilityHaste", skip_serializing_if = "Option::is_none")]
    pub ability_haste: Option<i32>,
    #[serde(rename = "abilityPower")]
    pub ability_power: i32,
    #[serde(rename = "armor")]
    pub armor: i32,
    #[serde(rename = "armorPen")]
    pub armor_pen: i32,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i32,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i32,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i32,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i32,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i32,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i32,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i32,
    #[serde(rename = "health")]
    pub health: i32,
    #[serde(rename = "healthMax")]
    pub health_max: i32,
    #[serde(rename = "healthRegen")]
    pub health_regen: i32,
    #[serde(rename = "lifesteal")]
    pub lifesteal: i32,
    #[serde(rename = "magicPen")]
    pub magic_pen: i32,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i32,
    #[serde(rename = "magicResist")]
    pub magic_resist: i32,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i32,
    #[serde(rename = "omnivamp", skip_serializing_if = "Option::is_none")]
    pub omnivamp: Option<i32>,
    #[serde(rename = "physicalVamp", skip_serializing_if = "Option::is_none")]
    pub physical_vamp: Option<i32>,
    #[serde(rename = "power")]
    pub power: i32,
    #[serde(rename = "powerMax")]
    pub power_max: i32,
    #[serde(rename = "powerRegen")]
    pub power_regen: i32,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i32,
}

impl MatchV5PeriodMatchTimelineInfoFrameParticipantFrameChampionStats {
    pub fn new(ability_power: i32, armor: i32, armor_pen: i32, armor_pen_percent: i32, attack_damage: i32, attack_speed: i32, bonus_armor_pen_percent: i32, bonus_magic_pen_percent: i32, cc_reduction: i32, cooldown_reduction: i32, health: i32, health_max: i32, health_regen: i32, lifesteal: i32, magic_pen: i32, magic_pen_percent: i32, magic_resist: i32, movement_speed: i32, power: i32, power_max: i32, power_regen: i32, spell_vamp: i32) -> MatchV5PeriodMatchTimelineInfoFrameParticipantFrameChampionStats {
        MatchV5PeriodMatchTimelineInfoFrameParticipantFrameChampionStats {
            ability_haste: None,
            ability_power,
            armor,
            armor_pen,
            armor_pen_percent,
            attack_damage,
            attack_speed,
            bonus_armor_pen_percent,
            bonus_magic_pen_percent,
            cc_reduction,
            cooldown_reduction,
            health,
            health_max,
            health_regen,
            lifesteal,
            magic_pen,
            magic_pen_percent,
            magic_resist,
            movement_speed,
            omnivamp: None,
            physical_vamp: None,
            power,
            power_max,
            power_regen,
            spell_vamp,
        }
    }
}


