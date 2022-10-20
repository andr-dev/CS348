




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodParticipantChallenges {
    #[serde(rename = "12AssistStreakCount", skip_serializing_if = "Option::is_none")]
    pub param_12_assist_streak_count: Option<f64>,
    #[serde(rename = "abilityUses", skip_serializing_if = "Option::is_none")]
    pub ability_uses: Option<f64>,
    #[serde(rename = "acesBefore15Minutes", skip_serializing_if = "Option::is_none")]
    pub aces_before15_minutes: Option<f64>,
    #[serde(rename = "alliedJungleMonsterKills", skip_serializing_if = "Option::is_none")]
    pub allied_jungle_monster_kills: Option<f64>,
    #[serde(rename = "baronBuffGoldAdvantageOverThreshold", skip_serializing_if = "Option::is_none")]
    pub baron_buff_gold_advantage_over_threshold: Option<f64>,
    #[serde(rename = "baronTakedowns", skip_serializing_if = "Option::is_none")]
    pub baron_takedowns: Option<f64>,
    #[serde(rename = "blastConeOppositeOpponentCount", skip_serializing_if = "Option::is_none")]
    pub blast_cone_opposite_opponent_count: Option<f64>,
    #[serde(rename = "bountyGold", skip_serializing_if = "Option::is_none")]
    pub bounty_gold: Option<f64>,
    #[serde(rename = "buffsStolen", skip_serializing_if = "Option::is_none")]
    pub buffs_stolen: Option<f64>,
    #[serde(rename = "completeSupportQuestInTime", skip_serializing_if = "Option::is_none")]
    pub complete_support_quest_in_time: Option<f64>,
    #[serde(rename = "controlWardsPlaced", skip_serializing_if = "Option::is_none")]
    pub control_wards_placed: Option<f64>,
    #[serde(rename = "controlWardTimeCoverageInRiverOrEnemyHalf", skip_serializing_if = "Option::is_none")]
    pub control_ward_time_coverage_in_river_or_enemy_half: Option<f64>,
    #[serde(rename = "damagePerMinute", skip_serializing_if = "Option::is_none")]
    pub damage_per_minute: Option<f64>,
    #[serde(rename = "damageTakenOnTeamPercentage", skip_serializing_if = "Option::is_none")]
    pub damage_taken_on_team_percentage: Option<f64>,
    #[serde(rename = "dancedWithRiftHerald", skip_serializing_if = "Option::is_none")]
    pub danced_with_rift_herald: Option<f64>,
    #[serde(rename = "deathsByEnemyChamps", skip_serializing_if = "Option::is_none")]
    pub deaths_by_enemy_champs: Option<f64>,
    #[serde(rename = "dodgeSkillShotsSmallWindow", skip_serializing_if = "Option::is_none")]
    pub dodge_skill_shots_small_window: Option<f64>,
    #[serde(rename = "doubleAces", skip_serializing_if = "Option::is_none")]
    pub double_aces: Option<f64>,
    #[serde(rename = "dragonTakedowns", skip_serializing_if = "Option::is_none")]
    pub dragon_takedowns: Option<f64>,
    #[serde(rename = "earliestBaron", skip_serializing_if = "Option::is_none")]
    pub earliest_baron: Option<f64>,
    #[serde(rename = "earliestDragonTakedown", skip_serializing_if = "Option::is_none")]
    pub earliest_dragon_takedown: Option<f64>,
    #[serde(rename = "earliestElderDragon", skip_serializing_if = "Option::is_none")]
    pub earliest_elder_dragon: Option<f64>,
    #[serde(rename = "earlyLaningPhaseGoldExpAdvantage", skip_serializing_if = "Option::is_none")]
    pub early_laning_phase_gold_exp_advantage: Option<f64>,
    #[serde(rename = "effectiveHealAndShielding", skip_serializing_if = "Option::is_none")]
    pub effective_heal_and_shielding: Option<f64>,
    #[serde(rename = "elderDragonKillsWithOpposingSoul", skip_serializing_if = "Option::is_none")]
    pub elder_dragon_kills_with_opposing_soul: Option<f64>,
    #[serde(rename = "elderDragonMultikills", skip_serializing_if = "Option::is_none")]
    pub elder_dragon_multikills: Option<f64>,
    #[serde(rename = "enemyChampionImmobilizations", skip_serializing_if = "Option::is_none")]
    pub enemy_champion_immobilizations: Option<f64>,
    #[serde(rename = "enemyJungleMonsterKills", skip_serializing_if = "Option::is_none")]
    pub enemy_jungle_monster_kills: Option<f64>,
    #[serde(rename = "epicMonsterKillsNearEnemyJungler", skip_serializing_if = "Option::is_none")]
    pub epic_monster_kills_near_enemy_jungler: Option<f64>,
    #[serde(rename = "epicMonsterKillsWithin30SecondsOfSpawn", skip_serializing_if = "Option::is_none")]
    pub epic_monster_kills_within30_seconds_of_spawn: Option<f64>,
    #[serde(rename = "epicMonsterSteals", skip_serializing_if = "Option::is_none")]
    pub epic_monster_steals: Option<f64>,
    #[serde(rename = "epicMonsterStolenWithoutSmite", skip_serializing_if = "Option::is_none")]
    pub epic_monster_stolen_without_smite: Option<f64>,
    #[serde(rename = "fasterSupportQuestCompletion", skip_serializing_if = "Option::is_none")]
    pub faster_support_quest_completion: Option<f64>,
    #[serde(rename = "fastestLegendary", skip_serializing_if = "Option::is_none")]
    pub fastest_legendary: Option<f64>,
    #[serde(rename = "firstTurretKilledTime", skip_serializing_if = "Option::is_none")]
    pub first_turret_killed_time: Option<f64>,
    #[serde(rename = "flawlessAces", skip_serializing_if = "Option::is_none")]
    pub flawless_aces: Option<f64>,
    #[serde(rename = "fullTeamTakedown", skip_serializing_if = "Option::is_none")]
    pub full_team_takedown: Option<f64>,
    #[serde(rename = "gameLength", skip_serializing_if = "Option::is_none")]
    pub game_length: Option<f64>,
    #[serde(rename = "getTakedownsInAllLanesEarlyJungleAsLaner", skip_serializing_if = "Option::is_none")]
    pub get_takedowns_in_all_lanes_early_jungle_as_laner: Option<f64>,
    #[serde(rename = "goldPerMinute", skip_serializing_if = "Option::is_none")]
    pub gold_per_minute: Option<f64>,
    #[serde(rename = "hadAfkTeammate", skip_serializing_if = "Option::is_none")]
    pub had_afk_teammate: Option<f64>,
    #[serde(rename = "hadOpenNexus", skip_serializing_if = "Option::is_none")]
    pub had_open_nexus: Option<f64>,
    #[serde(rename = "highestChampionDamage", skip_serializing_if = "Option::is_none")]
    pub highest_champion_damage: Option<f64>,
    #[serde(rename = "highestCrowdControlScore", skip_serializing_if = "Option::is_none")]
    pub highest_crowd_control_score: Option<f64>,
    #[serde(rename = "highestWardKills", skip_serializing_if = "Option::is_none")]
    pub highest_ward_kills: Option<f64>,
    #[serde(rename = "immobilizeAndKillWithAlly", skip_serializing_if = "Option::is_none")]
    pub immobilize_and_kill_with_ally: Option<f64>,
    #[serde(rename = "initialBuffCount", skip_serializing_if = "Option::is_none")]
    pub initial_buff_count: Option<f64>,
    #[serde(rename = "initialCrabCount", skip_serializing_if = "Option::is_none")]
    pub initial_crab_count: Option<f64>,
    #[serde(rename = "jungleCsBefore10Minutes", skip_serializing_if = "Option::is_none")]
    pub jungle_cs_before10_minutes: Option<f64>,
    #[serde(rename = "junglerKillsEarlyJungle", skip_serializing_if = "Option::is_none")]
    pub jungler_kills_early_jungle: Option<f64>,
    #[serde(rename = "junglerTakedownsNearDamagedEpicMonster", skip_serializing_if = "Option::is_none")]
    pub jungler_takedowns_near_damaged_epic_monster: Option<f64>,
    #[serde(rename = "kda", skip_serializing_if = "Option::is_none")]
    pub kda: Option<f64>,
    #[serde(rename = "killAfterHiddenWithAlly", skip_serializing_if = "Option::is_none")]
    pub kill_after_hidden_with_ally: Option<f64>,
    #[serde(rename = "killedChampTookFullTeamDamageSurvived", skip_serializing_if = "Option::is_none")]
    pub killed_champ_took_full_team_damage_survived: Option<f64>,
    #[serde(rename = "killParticipation", skip_serializing_if = "Option::is_none")]
    pub kill_participation: Option<f64>,
    #[serde(rename = "killsNearEnemyTurret", skip_serializing_if = "Option::is_none")]
    pub kills_near_enemy_turret: Option<f64>,
    #[serde(rename = "killsOnLanersEarlyJungleAsJungler", skip_serializing_if = "Option::is_none")]
    pub kills_on_laners_early_jungle_as_jungler: Option<f64>,
    #[serde(rename = "killsOnOtherLanesEarlyJungleAsLaner", skip_serializing_if = "Option::is_none")]
    pub kills_on_other_lanes_early_jungle_as_laner: Option<f64>,
    #[serde(rename = "killsOnRecentlyHealedByAramPack", skip_serializing_if = "Option::is_none")]
    pub kills_on_recently_healed_by_aram_pack: Option<f64>,
    #[serde(rename = "killsUnderOwnTurret", skip_serializing_if = "Option::is_none")]
    pub kills_under_own_turret: Option<f64>,
    #[serde(rename = "killsWithHelpFromEpicMonster", skip_serializing_if = "Option::is_none")]
    pub kills_with_help_from_epic_monster: Option<f64>,
    #[serde(rename = "knockEnemyIntoTeamAndKill", skip_serializing_if = "Option::is_none")]
    pub knock_enemy_into_team_and_kill: Option<f64>,
    #[serde(rename = "kTurretsDestroyedBeforePlatesFall", skip_serializing_if = "Option::is_none")]
    pub k_turrets_destroyed_before_plates_fall: Option<f64>,
    #[serde(rename = "landSkillShotsEarlyGame", skip_serializing_if = "Option::is_none")]
    pub land_skill_shots_early_game: Option<f64>,
    #[serde(rename = "laneMinionsFirst10Minutes", skip_serializing_if = "Option::is_none")]
    pub lane_minions_first10_minutes: Option<f64>,
    #[serde(rename = "laningPhaseGoldExpAdvantage", skip_serializing_if = "Option::is_none")]
    pub laning_phase_gold_exp_advantage: Option<f64>,
    #[serde(rename = "legendaryCount", skip_serializing_if = "Option::is_none")]
    pub legendary_count: Option<f64>,
    #[serde(rename = "lostAnInhibitor", skip_serializing_if = "Option::is_none")]
    pub lost_an_inhibitor: Option<f64>,
    #[serde(rename = "maxCsAdvantageOnLaneOpponent", skip_serializing_if = "Option::is_none")]
    pub max_cs_advantage_on_lane_opponent: Option<f64>,
    #[serde(rename = "maxKillDeficit", skip_serializing_if = "Option::is_none")]
    pub max_kill_deficit: Option<f64>,
    #[serde(rename = "maxLevelLeadLaneOpponent", skip_serializing_if = "Option::is_none")]
    pub max_level_lead_lane_opponent: Option<f64>,
    #[serde(rename = "mejaisFullStackInTime", skip_serializing_if = "Option::is_none")]
    pub mejais_full_stack_in_time: Option<f64>,
    #[serde(rename = "moreEnemyJungleThanOpponent", skip_serializing_if = "Option::is_none")]
    pub more_enemy_jungle_than_opponent: Option<f64>,
    #[serde(rename = "mostWardsDestroyedOneSweeper", skip_serializing_if = "Option::is_none")]
    pub most_wards_destroyed_one_sweeper: Option<f64>,
    #[serde(rename = "multiKillOneSpell", skip_serializing_if = "Option::is_none")]
    pub multi_kill_one_spell: Option<f64>,
    #[serde(rename = "multikills", skip_serializing_if = "Option::is_none")]
    pub multikills: Option<f64>,
    #[serde(rename = "multikillsAfterAggressiveFlash", skip_serializing_if = "Option::is_none")]
    pub multikills_after_aggressive_flash: Option<f64>,
    #[serde(rename = "multiTurretRiftHeraldCount", skip_serializing_if = "Option::is_none")]
    pub multi_turret_rift_herald_count: Option<f64>,
    #[serde(rename = "mythicItemUsed", skip_serializing_if = "Option::is_none")]
    pub mythic_item_used: Option<f64>,
    #[serde(rename = "outerTurretExecutesBefore10Minutes", skip_serializing_if = "Option::is_none")]
    pub outer_turret_executes_before10_minutes: Option<f64>,
    #[serde(rename = "outnumberedKills", skip_serializing_if = "Option::is_none")]
    pub outnumbered_kills: Option<f64>,
    #[serde(rename = "outnumberedNexusKill", skip_serializing_if = "Option::is_none")]
    pub outnumbered_nexus_kill: Option<f64>,
    #[serde(rename = "perfectDragonSoulsTaken", skip_serializing_if = "Option::is_none")]
    pub perfect_dragon_souls_taken: Option<f64>,
    #[serde(rename = "perfectGame", skip_serializing_if = "Option::is_none")]
    pub perfect_game: Option<f64>,
    #[serde(rename = "pickKillWithAlly", skip_serializing_if = "Option::is_none")]
    pub pick_kill_with_ally: Option<f64>,
    #[serde(rename = "poroExplosions", skip_serializing_if = "Option::is_none")]
    pub poro_explosions: Option<f64>,
    #[serde(rename = "quickCleanse", skip_serializing_if = "Option::is_none")]
    pub quick_cleanse: Option<f64>,
    #[serde(rename = "quickFirstTurret", skip_serializing_if = "Option::is_none")]
    pub quick_first_turret: Option<f64>,
    #[serde(rename = "quickSoloKills", skip_serializing_if = "Option::is_none")]
    pub quick_solo_kills: Option<f64>,
    #[serde(rename = "riftHeraldTakedowns", skip_serializing_if = "Option::is_none")]
    pub rift_herald_takedowns: Option<f64>,
    #[serde(rename = "saveAllyFromDeath", skip_serializing_if = "Option::is_none")]
    pub save_ally_from_death: Option<f64>,
    #[serde(rename = "scuttleCrabKills", skip_serializing_if = "Option::is_none")]
    pub scuttle_crab_kills: Option<f64>,
    #[serde(rename = "shortestTimeToAceFromFirstTakedown", skip_serializing_if = "Option::is_none")]
    pub shortest_time_to_ace_from_first_takedown: Option<f64>,
    #[serde(rename = "skillshotsDodged", skip_serializing_if = "Option::is_none")]
    pub skillshots_dodged: Option<f64>,
    #[serde(rename = "skillshotsHit", skip_serializing_if = "Option::is_none")]
    pub skillshots_hit: Option<f64>,
    #[serde(rename = "snowballsHit", skip_serializing_if = "Option::is_none")]
    pub snowballs_hit: Option<f64>,
    #[serde(rename = "soloBaronKills", skip_serializing_if = "Option::is_none")]
    pub solo_baron_kills: Option<f64>,
    #[serde(rename = "soloKills", skip_serializing_if = "Option::is_none")]
    pub solo_kills: Option<f64>,
    #[serde(rename = "soloTurretsLategame", skip_serializing_if = "Option::is_none")]
    pub solo_turrets_lategame: Option<f64>,
    #[serde(rename = "stealthWardsPlaced", skip_serializing_if = "Option::is_none")]
    pub stealth_wards_placed: Option<f64>,
    #[serde(rename = "survivedSingleDigitHpCount", skip_serializing_if = "Option::is_none")]
    pub survived_single_digit_hp_count: Option<f64>,
    #[serde(rename = "survivedThreeImmobilizesInFight", skip_serializing_if = "Option::is_none")]
    pub survived_three_immobilizes_in_fight: Option<f64>,
    #[serde(rename = "takedownOnFirstTurret", skip_serializing_if = "Option::is_none")]
    pub takedown_on_first_turret: Option<f64>,
    #[serde(rename = "takedowns", skip_serializing_if = "Option::is_none")]
    pub takedowns: Option<f64>,
    #[serde(rename = "takedownsAfterGainingLevelAdvantage", skip_serializing_if = "Option::is_none")]
    pub takedowns_after_gaining_level_advantage: Option<f64>,
    #[serde(rename = "takedownsBeforeJungleMinionSpawn", skip_serializing_if = "Option::is_none")]
    pub takedowns_before_jungle_minion_spawn: Option<f64>,
    #[serde(rename = "takedownsFirst25Minutes", skip_serializing_if = "Option::is_none")]
    pub takedowns_first25_minutes: Option<f64>,
    #[serde(rename = "takedownsInAlcove", skip_serializing_if = "Option::is_none")]
    pub takedowns_in_alcove: Option<f64>,
    #[serde(rename = "takedownsInEnemyFountain", skip_serializing_if = "Option::is_none")]
    pub takedowns_in_enemy_fountain: Option<f64>,
    #[serde(rename = "teamBaronKills", skip_serializing_if = "Option::is_none")]
    pub team_baron_kills: Option<f64>,
    #[serde(rename = "teamDamagePercentage", skip_serializing_if = "Option::is_none")]
    pub team_damage_percentage: Option<f64>,
    #[serde(rename = "teamElderDragonKills", skip_serializing_if = "Option::is_none")]
    pub team_elder_dragon_kills: Option<f64>,
    #[serde(rename = "teamRiftHeraldKills", skip_serializing_if = "Option::is_none")]
    pub team_rift_herald_kills: Option<f64>,
    #[serde(rename = "teleportTakedowns", skip_serializing_if = "Option::is_none")]
    pub teleport_takedowns: Option<f64>,
    #[serde(rename = "thirdInhibitorDestroyedTime", skip_serializing_if = "Option::is_none")]
    pub third_inhibitor_destroyed_time: Option<f64>,
    #[serde(rename = "threeWardsOneSweeperCount", skip_serializing_if = "Option::is_none")]
    pub three_wards_one_sweeper_count: Option<f64>,
    #[serde(rename = "tookLargeDamageSurvived", skip_serializing_if = "Option::is_none")]
    pub took_large_damage_survived: Option<f64>,
    #[serde(rename = "turretPlatesTaken", skip_serializing_if = "Option::is_none")]
    pub turret_plates_taken: Option<f64>,
    #[serde(rename = "turretsTakenWithRiftHerald", skip_serializing_if = "Option::is_none")]
    pub turrets_taken_with_rift_herald: Option<f64>,
    #[serde(rename = "turretTakedowns", skip_serializing_if = "Option::is_none")]
    pub turret_takedowns: Option<f64>,
    #[serde(rename = "twentyMinionsIn3SecondsCount", skip_serializing_if = "Option::is_none")]
    pub twenty_minions_in3_seconds_count: Option<f64>,
    #[serde(rename = "unseenRecalls", skip_serializing_if = "Option::is_none")]
    pub unseen_recalls: Option<f64>,
    #[serde(rename = "visionScoreAdvantageLaneOpponent", skip_serializing_if = "Option::is_none")]
    pub vision_score_advantage_lane_opponent: Option<f64>,
    #[serde(rename = "visionScorePerMinute", skip_serializing_if = "Option::is_none")]
    pub vision_score_per_minute: Option<f64>,
    #[serde(rename = "wardsGuarded", skip_serializing_if = "Option::is_none")]
    pub wards_guarded: Option<f64>,
    #[serde(rename = "wardTakedowns", skip_serializing_if = "Option::is_none")]
    pub ward_takedowns: Option<f64>,
    #[serde(rename = "wardTakedownsBefore20M", skip_serializing_if = "Option::is_none")]
    pub ward_takedowns_before20_m: Option<f64>,
}

impl MatchV5PeriodParticipantChallenges {
    pub fn new() -> MatchV5PeriodParticipantChallenges {
        MatchV5PeriodParticipantChallenges {
            param_12_assist_streak_count: None,
            ability_uses: None,
            aces_before15_minutes: None,
            allied_jungle_monster_kills: None,
            baron_buff_gold_advantage_over_threshold: None,
            baron_takedowns: None,
            blast_cone_opposite_opponent_count: None,
            bounty_gold: None,
            buffs_stolen: None,
            complete_support_quest_in_time: None,
            control_wards_placed: None,
            control_ward_time_coverage_in_river_or_enemy_half: None,
            damage_per_minute: None,
            damage_taken_on_team_percentage: None,
            danced_with_rift_herald: None,
            deaths_by_enemy_champs: None,
            dodge_skill_shots_small_window: None,
            double_aces: None,
            dragon_takedowns: None,
            earliest_baron: None,
            earliest_dragon_takedown: None,
            earliest_elder_dragon: None,
            early_laning_phase_gold_exp_advantage: None,
            effective_heal_and_shielding: None,
            elder_dragon_kills_with_opposing_soul: None,
            elder_dragon_multikills: None,
            enemy_champion_immobilizations: None,
            enemy_jungle_monster_kills: None,
            epic_monster_kills_near_enemy_jungler: None,
            epic_monster_kills_within30_seconds_of_spawn: None,
            epic_monster_steals: None,
            epic_monster_stolen_without_smite: None,
            faster_support_quest_completion: None,
            fastest_legendary: None,
            first_turret_killed_time: None,
            flawless_aces: None,
            full_team_takedown: None,
            game_length: None,
            get_takedowns_in_all_lanes_early_jungle_as_laner: None,
            gold_per_minute: None,
            had_afk_teammate: None,
            had_open_nexus: None,
            highest_champion_damage: None,
            highest_crowd_control_score: None,
            highest_ward_kills: None,
            immobilize_and_kill_with_ally: None,
            initial_buff_count: None,
            initial_crab_count: None,
            jungle_cs_before10_minutes: None,
            jungler_kills_early_jungle: None,
            jungler_takedowns_near_damaged_epic_monster: None,
            kda: None,
            kill_after_hidden_with_ally: None,
            killed_champ_took_full_team_damage_survived: None,
            kill_participation: None,
            kills_near_enemy_turret: None,
            kills_on_laners_early_jungle_as_jungler: None,
            kills_on_other_lanes_early_jungle_as_laner: None,
            kills_on_recently_healed_by_aram_pack: None,
            kills_under_own_turret: None,
            kills_with_help_from_epic_monster: None,
            knock_enemy_into_team_and_kill: None,
            k_turrets_destroyed_before_plates_fall: None,
            land_skill_shots_early_game: None,
            lane_minions_first10_minutes: None,
            laning_phase_gold_exp_advantage: None,
            legendary_count: None,
            lost_an_inhibitor: None,
            max_cs_advantage_on_lane_opponent: None,
            max_kill_deficit: None,
            max_level_lead_lane_opponent: None,
            mejais_full_stack_in_time: None,
            more_enemy_jungle_than_opponent: None,
            most_wards_destroyed_one_sweeper: None,
            multi_kill_one_spell: None,
            multikills: None,
            multikills_after_aggressive_flash: None,
            multi_turret_rift_herald_count: None,
            mythic_item_used: None,
            outer_turret_executes_before10_minutes: None,
            outnumbered_kills: None,
            outnumbered_nexus_kill: None,
            perfect_dragon_souls_taken: None,
            perfect_game: None,
            pick_kill_with_ally: None,
            poro_explosions: None,
            quick_cleanse: None,
            quick_first_turret: None,
            quick_solo_kills: None,
            rift_herald_takedowns: None,
            save_ally_from_death: None,
            scuttle_crab_kills: None,
            shortest_time_to_ace_from_first_takedown: None,
            skillshots_dodged: None,
            skillshots_hit: None,
            snowballs_hit: None,
            solo_baron_kills: None,
            solo_kills: None,
            solo_turrets_lategame: None,
            stealth_wards_placed: None,
            survived_single_digit_hp_count: None,
            survived_three_immobilizes_in_fight: None,
            takedown_on_first_turret: None,
            takedowns: None,
            takedowns_after_gaining_level_advantage: None,
            takedowns_before_jungle_minion_spawn: None,
            takedowns_first25_minutes: None,
            takedowns_in_alcove: None,
            takedowns_in_enemy_fountain: None,
            team_baron_kills: None,
            team_damage_percentage: None,
            team_elder_dragon_kills: None,
            team_rift_herald_kills: None,
            teleport_takedowns: None,
            third_inhibitor_destroyed_time: None,
            three_wards_one_sweeper_count: None,
            took_large_damage_survived: None,
            turret_plates_taken: None,
            turrets_taken_with_rift_herald: None,
            turret_takedowns: None,
            twenty_minions_in3_seconds_count: None,
            unseen_recalls: None,
            vision_score_advantage_lane_opponent: None,
            vision_score_per_minute: None,
            wards_guarded: None,
            ward_takedowns: None,
            ward_takedowns_before20_m: None,
        }
    }
}


