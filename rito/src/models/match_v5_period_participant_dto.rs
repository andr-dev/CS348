




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodParticipantDto {
    #[serde(rename = "assists")]
    pub assists: i32,
    #[serde(rename = "baronKills")]
    pub baron_kills: i32,
    #[serde(rename = "bountyLevel")]
    pub bounty_level: i32,
    #[serde(rename = "champExperience")]
    pub champ_experience: i32,
    #[serde(rename = "champLevel")]
    pub champ_level: i32,
    /// Prior to patch 11.4, on Feb 18th, 2021, this field returned invalid championIds. We recommend determining the champion based on the championName field for matches played prior to patch 11.4.
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championName")]
    pub champion_name: String,
    /// This field is currently only utilized for Kayn's transformations. (Legal values: 0 - None, 1 - Slayer, 2 - Assassin)
    #[serde(rename = "championTransform")]
    pub champion_transform: i32,
    #[serde(rename = "consumablesPurchased")]
    pub consumables_purchased: i32,
    #[serde(rename = "damageDealtToBuildings", skip_serializing_if = "Option::is_none")]
    pub damage_dealt_to_buildings: Option<i32>,
    #[serde(rename = "damageDealtToObjectives")]
    pub damage_dealt_to_objectives: i32,
    #[serde(rename = "damageDealtToTurrets")]
    pub damage_dealt_to_turrets: i32,
    #[serde(rename = "damageSelfMitigated")]
    pub damage_self_mitigated: i32,
    #[serde(rename = "deaths")]
    pub deaths: i32,
    #[serde(rename = "detectorWardsPlaced")]
    pub detector_wards_placed: i32,
    #[serde(rename = "doubleKills")]
    pub double_kills: i32,
    #[serde(rename = "dragonKills")]
    pub dragon_kills: i32,
    #[serde(rename = "firstBloodAssist")]
    pub first_blood_assist: bool,
    #[serde(rename = "firstBloodKill")]
    pub first_blood_kill: bool,
    #[serde(rename = "firstTowerAssist")]
    pub first_tower_assist: bool,
    #[serde(rename = "firstTowerKill")]
    pub first_tower_kill: bool,
    #[serde(rename = "gameEndedInEarlySurrender")]
    pub game_ended_in_early_surrender: bool,
    #[serde(rename = "gameEndedInSurrender")]
    pub game_ended_in_surrender: bool,
    #[serde(rename = "goldEarned")]
    pub gold_earned: i32,
    #[serde(rename = "goldSpent")]
    pub gold_spent: i32,
    /// Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.
    #[serde(rename = "individualPosition")]
    pub individual_position: String,
    #[serde(rename = "inhibitorKills")]
    pub inhibitor_kills: i32,
    #[serde(rename = "inhibitorTakedowns", skip_serializing_if = "Option::is_none")]
    pub inhibitor_takedowns: Option<i32>,
    #[serde(rename = "inhibitorsLost", skip_serializing_if = "Option::is_none")]
    pub inhibitors_lost: Option<i32>,
    #[serde(rename = "item0")]
    pub item0: i32,
    #[serde(rename = "item1")]
    pub item1: i32,
    #[serde(rename = "item2")]
    pub item2: i32,
    #[serde(rename = "item3")]
    pub item3: i32,
    #[serde(rename = "item4")]
    pub item4: i32,
    #[serde(rename = "item5")]
    pub item5: i32,
    #[serde(rename = "item6")]
    pub item6: i32,
    #[serde(rename = "itemsPurchased")]
    pub items_purchased: i32,
    #[serde(rename = "killingSprees")]
    pub killing_sprees: i32,
    #[serde(rename = "kills")]
    pub kills: i32,
    #[serde(rename = "lane")]
    pub lane: String,
    #[serde(rename = "largestCriticalStrike")]
    pub largest_critical_strike: i32,
    #[serde(rename = "largestKillingSpree")]
    pub largest_killing_spree: i32,
    #[serde(rename = "largestMultiKill")]
    pub largest_multi_kill: i32,
    #[serde(rename = "longestTimeSpentLiving")]
    pub longest_time_spent_living: i32,
    #[serde(rename = "magicDamageDealt")]
    pub magic_damage_dealt: i32,
    #[serde(rename = "magicDamageDealtToChampions")]
    pub magic_damage_dealt_to_champions: i32,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i32,
    #[serde(rename = "neutralMinionsKilled")]
    pub neutral_minions_killed: i32,
    #[serde(rename = "nexusKills")]
    pub nexus_kills: i32,
    #[serde(rename = "nexusTakedowns", skip_serializing_if = "Option::is_none")]
    pub nexus_takedowns: Option<i32>,
    #[serde(rename = "nexusLost", skip_serializing_if = "Option::is_none")]
    pub nexus_lost: Option<i32>,
    #[serde(rename = "objectivesStolen")]
    pub objectives_stolen: i32,
    #[serde(rename = "objectivesStolenAssists")]
    pub objectives_stolen_assists: i32,
    #[serde(rename = "participantId")]
    pub participant_id: i32,
    #[serde(rename = "pentaKills")]
    pub penta_kills: i32,
    #[serde(rename = "perks")]
    pub perks: Box<crate::models::MatchV5PeriodPerksDto>,
    #[serde(rename = "physicalDamageDealt")]
    pub physical_damage_dealt: i32,
    #[serde(rename = "physicalDamageDealtToChampions")]
    pub physical_damage_dealt_to_champions: i32,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i32,
    #[serde(rename = "profileIcon")]
    pub profile_icon: i32,
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "quadraKills")]
    pub quadra_kills: i32,
    #[serde(rename = "riotIdName")]
    pub riot_id_name: String,
    #[serde(rename = "riotIdTagline")]
    pub riot_id_tagline: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "sightWardsBoughtInGame")]
    pub sight_wards_bought_in_game: i32,
    #[serde(rename = "spell1Casts")]
    pub spell1_casts: i32,
    #[serde(rename = "spell2Casts")]
    pub spell2_casts: i32,
    #[serde(rename = "spell3Casts")]
    pub spell3_casts: i32,
    #[serde(rename = "spell4Casts")]
    pub spell4_casts: i32,
    #[serde(rename = "summoner1Casts")]
    pub summoner1_casts: i32,
    #[serde(rename = "summoner1Id")]
    pub summoner1_id: i32,
    #[serde(rename = "summoner2Casts")]
    pub summoner2_casts: i32,
    #[serde(rename = "summoner2Id")]
    pub summoner2_id: i32,
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i32,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "teamEarlySurrendered")]
    pub team_early_surrendered: bool,
    #[serde(rename = "teamId")]
    pub team_id: i32,
    /// Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.
    #[serde(rename = "teamPosition")]
    pub team_position: String,
    #[serde(rename = "timeCCingOthers")]
    pub time_c_cing_others: i32,
    #[serde(rename = "timePlayed")]
    pub time_played: i32,
    #[serde(rename = "totalDamageDealt")]
    pub total_damage_dealt: i32,
    #[serde(rename = "totalDamageDealtToChampions")]
    pub total_damage_dealt_to_champions: i32,
    #[serde(rename = "totalDamageShieldedOnTeammates")]
    pub total_damage_shielded_on_teammates: i32,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i32,
    #[serde(rename = "totalHeal")]
    pub total_heal: i32,
    #[serde(rename = "totalHealsOnTeammates")]
    pub total_heals_on_teammates: i32,
    #[serde(rename = "totalMinionsKilled")]
    pub total_minions_killed: i32,
    #[serde(rename = "totalTimeCCDealt")]
    pub total_time_cc_dealt: i32,
    #[serde(rename = "totalTimeSpentDead")]
    pub total_time_spent_dead: i32,
    #[serde(rename = "totalUnitsHealed")]
    pub total_units_healed: i32,
    #[serde(rename = "tripleKills")]
    pub triple_kills: i32,
    #[serde(rename = "trueDamageDealt")]
    pub true_damage_dealt: i32,
    #[serde(rename = "trueDamageDealtToChampions")]
    pub true_damage_dealt_to_champions: i32,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i32,
    #[serde(rename = "turretKills")]
    pub turret_kills: i32,
    #[serde(rename = "turretTakedowns", skip_serializing_if = "Option::is_none")]
    pub turret_takedowns: Option<i32>,
    #[serde(rename = "turretsLost", skip_serializing_if = "Option::is_none")]
    pub turrets_lost: Option<i32>,
    #[serde(rename = "unrealKills")]
    pub unreal_kills: i32,
    #[serde(rename = "visionScore")]
    pub vision_score: i32,
    #[serde(rename = "visionWardsBoughtInGame")]
    pub vision_wards_bought_in_game: i32,
    #[serde(rename = "wardsKilled")]
    pub wards_killed: i32,
    #[serde(rename = "wardsPlaced")]
    pub wards_placed: i32,
    #[serde(rename = "win")]
    pub win: bool,
    #[serde(rename = "challenges", skip_serializing_if = "Option::is_none")]
    pub challenges: Option<Box<crate::models::MatchV5PeriodParticipantChallenges>>,
}

impl MatchV5PeriodParticipantDto {
    pub fn new(assists: i32, baron_kills: i32, bounty_level: i32, champ_experience: i32, champ_level: i32, champion_id: i32, champion_name: String, champion_transform: i32, consumables_purchased: i32, damage_dealt_to_objectives: i32, damage_dealt_to_turrets: i32, damage_self_mitigated: i32, deaths: i32, detector_wards_placed: i32, double_kills: i32, dragon_kills: i32, first_blood_assist: bool, first_blood_kill: bool, first_tower_assist: bool, first_tower_kill: bool, game_ended_in_early_surrender: bool, game_ended_in_surrender: bool, gold_earned: i32, gold_spent: i32, individual_position: String, inhibitor_kills: i32, item0: i32, item1: i32, item2: i32, item3: i32, item4: i32, item5: i32, item6: i32, items_purchased: i32, killing_sprees: i32, kills: i32, lane: String, largest_critical_strike: i32, largest_killing_spree: i32, largest_multi_kill: i32, longest_time_spent_living: i32, magic_damage_dealt: i32, magic_damage_dealt_to_champions: i32, magic_damage_taken: i32, neutral_minions_killed: i32, nexus_kills: i32, objectives_stolen: i32, objectives_stolen_assists: i32, participant_id: i32, penta_kills: i32, perks: crate::models::MatchV5PeriodPerksDto, physical_damage_dealt: i32, physical_damage_dealt_to_champions: i32, physical_damage_taken: i32, profile_icon: i32, puuid: String, quadra_kills: i32, riot_id_name: String, riot_id_tagline: String, role: String, sight_wards_bought_in_game: i32, spell1_casts: i32, spell2_casts: i32, spell3_casts: i32, spell4_casts: i32, summoner1_casts: i32, summoner1_id: i32, summoner2_casts: i32, summoner2_id: i32, summoner_id: String, summoner_level: i32, summoner_name: String, team_early_surrendered: bool, team_id: i32, team_position: String, time_c_cing_others: i32, time_played: i32, total_damage_dealt: i32, total_damage_dealt_to_champions: i32, total_damage_shielded_on_teammates: i32, total_damage_taken: i32, total_heal: i32, total_heals_on_teammates: i32, total_minions_killed: i32, total_time_cc_dealt: i32, total_time_spent_dead: i32, total_units_healed: i32, triple_kills: i32, true_damage_dealt: i32, true_damage_dealt_to_champions: i32, true_damage_taken: i32, turret_kills: i32, unreal_kills: i32, vision_score: i32, vision_wards_bought_in_game: i32, wards_killed: i32, wards_placed: i32, win: bool) -> MatchV5PeriodParticipantDto {
        MatchV5PeriodParticipantDto {
            assists,
            baron_kills,
            bounty_level,
            champ_experience,
            champ_level,
            champion_id,
            champion_name,
            champion_transform,
            consumables_purchased,
            damage_dealt_to_buildings: None,
            damage_dealt_to_objectives,
            damage_dealt_to_turrets,
            damage_self_mitigated,
            deaths,
            detector_wards_placed,
            double_kills,
            dragon_kills,
            first_blood_assist,
            first_blood_kill,
            first_tower_assist,
            first_tower_kill,
            game_ended_in_early_surrender,
            game_ended_in_surrender,
            gold_earned,
            gold_spent,
            individual_position,
            inhibitor_kills,
            inhibitor_takedowns: None,
            inhibitors_lost: None,
            item0,
            item1,
            item2,
            item3,
            item4,
            item5,
            item6,
            items_purchased,
            killing_sprees,
            kills,
            lane,
            largest_critical_strike,
            largest_killing_spree,
            largest_multi_kill,
            longest_time_spent_living,
            magic_damage_dealt,
            magic_damage_dealt_to_champions,
            magic_damage_taken,
            neutral_minions_killed,
            nexus_kills,
            nexus_takedowns: None,
            nexus_lost: None,
            objectives_stolen,
            objectives_stolen_assists,
            participant_id,
            penta_kills,
            perks: Box::new(perks),
            physical_damage_dealt,
            physical_damage_dealt_to_champions,
            physical_damage_taken,
            profile_icon,
            puuid,
            quadra_kills,
            riot_id_name,
            riot_id_tagline,
            role,
            sight_wards_bought_in_game,
            spell1_casts,
            spell2_casts,
            spell3_casts,
            spell4_casts,
            summoner1_casts,
            summoner1_id,
            summoner2_casts,
            summoner2_id,
            summoner_id,
            summoner_level,
            summoner_name,
            team_early_surrendered,
            team_id,
            team_position,
            time_c_cing_others,
            time_played,
            total_damage_dealt,
            total_damage_dealt_to_champions,
            total_damage_shielded_on_teammates,
            total_damage_taken,
            total_heal,
            total_heals_on_teammates,
            total_minions_killed,
            total_time_cc_dealt,
            total_time_spent_dead,
            total_units_healed,
            triple_kills,
            true_damage_dealt,
            true_damage_dealt_to_champions,
            true_damage_taken,
            turret_kills,
            turret_takedowns: None,
            turrets_lost: None,
            unreal_kills,
            vision_score,
            vision_wards_bought_in_game,
            wards_killed,
            wards_placed,
            win,
            challenges: None,
        }
    }
}


