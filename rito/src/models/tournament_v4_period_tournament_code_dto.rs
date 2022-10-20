




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentV4PeriodTournamentCodeDto {
    /// The tournament code.
    #[serde(rename = "code")]
    pub code: String,
    /// The spectator mode for the tournament code game.
    #[serde(rename = "spectators")]
    pub spectators: String,
    /// The lobby name for the tournament code game.
    #[serde(rename = "lobbyName")]
    pub lobby_name: String,
    /// The metadata for tournament code.
    #[serde(rename = "metaData")]
    pub meta_data: String,
    /// The password for the tournament code game.
    #[serde(rename = "password")]
    pub password: String,
    /// The team size for the tournament code game.
    #[serde(rename = "teamSize")]
    pub team_size: i32,
    /// The provider's ID.
    #[serde(rename = "providerId")]
    pub provider_id: i32,
    /// The pick mode for tournament code game.
    #[serde(rename = "pickType")]
    pub pick_type: String,
    /// The tournament's ID.
    #[serde(rename = "tournamentId")]
    pub tournament_id: i32,
    /// The tournament code's ID.
    #[serde(rename = "id")]
    pub id: i32,
    /// The tournament code's region.              (Legal values:  BR,  EUNE,  EUW,  JP,  LAN,  LAS,  NA,  OCE,  PBE,  RU,  TR)
    #[serde(rename = "region")]
    pub region: Region,
    /// The game map for the tournament code game
    #[serde(rename = "map")]
    pub map: String,
    /// The summonerIds of the participants (Encrypted)
    #[serde(rename = "participants")]
    pub participants: Vec<String>,
}

impl TournamentV4PeriodTournamentCodeDto {
    pub fn new(code: String, spectators: String, lobby_name: String, meta_data: String, password: String, team_size: i32, provider_id: i32, pick_type: String, tournament_id: i32, id: i32, region: Region, map: String, participants: Vec<String>) -> TournamentV4PeriodTournamentCodeDto {
        TournamentV4PeriodTournamentCodeDto {
            code,
            spectators,
            lobby_name,
            meta_data,
            password,
            team_size,
            provider_id,
            pick_type,
            tournament_id,
            id,
            region,
            map,
            participants,
        }
    }
}

/// The tournament code's region.              (Legal values:  BR,  EUNE,  EUW,  JP,  LAN,  LAS,  NA,  OCE,  PBE,  RU,  TR)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "EUNE")]
    Eune,
    #[serde(rename = "EUW")]
    Euw,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "LAN")]
    Lan,
    #[serde(rename = "LAS")]
    Las,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "OCE")]
    Oce,
    #[serde(rename = "PBE")]
    Pbe,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "TR")]
    Tr,
}

impl Default for Region {
    fn default() -> Region {
        Self::Br
    }
}

