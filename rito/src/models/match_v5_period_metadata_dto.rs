




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchV5PeriodMetadataDto {
    /// Match data version.
    #[serde(rename = "dataVersion")]
    pub data_version: String,
    /// Match id.
    #[serde(rename = "matchId")]
    pub match_id: String,
    /// A list of participant PUUIDs.
    #[serde(rename = "participants")]
    pub participants: Vec<String>,
}

impl MatchV5PeriodMetadataDto {
    pub fn new(data_version: String, match_id: String, participants: Vec<String>) -> MatchV5PeriodMetadataDto {
        MatchV5PeriodMetadataDto {
            data_version,
            match_id,
            participants,
        }
    }
}


