#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TournamentStubV4PeriodProviderRegistrationParameters {
    /// The region in which the provider will be running tournaments.              (Legal values:  BR,  EUNE,  EUW,  JP,  LAN,  LAS,  NA,  OCE,  PBE,  RU,  TR)
    #[serde(rename = "region")]
    pub region: Region,
    /// The provider's callback URL to which tournament game results in this region should be posted. The URL must be well-formed, use the http or https protocol, and use the default port for the protocol (http URLs must use port 80, https URLs must use port 443).
    #[serde(rename = "url")]
    pub url: String,
}

impl TournamentStubV4PeriodProviderRegistrationParameters {
    pub fn new(
        region: Region,
        url: String,
    ) -> TournamentStubV4PeriodProviderRegistrationParameters {
        TournamentStubV4PeriodProviderRegistrationParameters { region, url }
    }
}

/// The region in which the provider will be running tournaments.              (Legal values:  BR,  EUNE,  EUW,  JP,  LAN,  LAS,  NA,  OCE,  PBE,  RU,  TR)
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
