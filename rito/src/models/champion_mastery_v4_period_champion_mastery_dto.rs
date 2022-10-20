/// ChampionMasteryV4PeriodChampionMasteryDto : This object contains single Champion Mastery information for player and champion combination.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChampionMasteryV4PeriodChampionMasteryDto {
    /// Number of points needed to achieve next level. Zero if player reached maximum champion level for this champion.
    #[serde(rename = "championPointsUntilNextLevel")]
    pub champion_points_until_next_level: i64,
    /// Is chest granted for this champion or not in current season.
    #[serde(rename = "chestGranted")]
    pub chest_granted: bool,
    /// Champion ID for this entry.
    #[serde(rename = "championId")]
    pub champion_id: i64,
    /// Last time this champion was played by this player - in Unix milliseconds time format.
    #[serde(rename = "lastPlayTime")]
    pub last_play_time: i64,
    /// Champion level for specified player and champion combination.
    #[serde(rename = "championLevel")]
    pub champion_level: i32,
    /// Summoner ID for this entry. (Encrypted)
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    /// Total number of champion points for this player and champion combination - they are used to determine championLevel.
    #[serde(rename = "championPoints")]
    pub champion_points: i32,
    /// Number of points earned since current level has been achieved.
    #[serde(rename = "championPointsSinceLastLevel")]
    pub champion_points_since_last_level: i64,
    /// The token earned for this champion at the current championLevel. When the championLevel is advanced the tokensEarned resets to 0.
    #[serde(rename = "tokensEarned")]
    pub tokens_earned: i32,
}

impl ChampionMasteryV4PeriodChampionMasteryDto {
    /// This object contains single Champion Mastery information for player and champion combination.
    pub fn new(
        champion_points_until_next_level: i64,
        chest_granted: bool,
        champion_id: i64,
        last_play_time: i64,
        champion_level: i32,
        summoner_id: String,
        champion_points: i32,
        champion_points_since_last_level: i64,
        tokens_earned: i32,
    ) -> ChampionMasteryV4PeriodChampionMasteryDto {
        ChampionMasteryV4PeriodChampionMasteryDto {
            champion_points_until_next_level,
            chest_granted,
            champion_id,
            last_play_time,
            champion_level,
            summoner_id,
            champion_points,
            champion_points_since_last_level,
            tokens_earned,
        }
    }
}
