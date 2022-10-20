




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorMatchV1PeriodMetadataDto {
    /// Match data version.
    #[serde(rename = "data_version")]
    pub data_version: String,
    /// Match id.
    #[serde(rename = "match_id")]
    pub match_id: String,
    /// A list of participant PUUIDs.
    #[serde(rename = "participants")]
    pub participants: Vec<String>,
}

impl LorMatchV1PeriodMetadataDto {
    pub fn new(data_version: String, match_id: String, participants: Vec<String>) -> LorMatchV1PeriodMetadataDto {
        LorMatchV1PeriodMetadataDto {
            data_version,
            match_id,
            participants,
        }
    }
}


