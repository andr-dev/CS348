

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValStatusV1PeriodUpdateDto {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "publish")]
    pub publish: bool,
    /// (Legal values: riotclient, riotstatus, game)
    #[serde(rename = "publish_locations")]
    pub publish_locations: Vec<PublishLocations>,
    #[serde(rename = "translations")]
    pub translations: Vec<crate::models::ValStatusV1PeriodContentDto>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ValStatusV1PeriodUpdateDto {
    pub fn new(
        id: i32,
        author: String,
        publish: bool,
        publish_locations: Vec<PublishLocations>,
        translations: Vec<crate::models::ValStatusV1PeriodContentDto>,
        created_at: String,
        updated_at: String,
    ) -> ValStatusV1PeriodUpdateDto {
        ValStatusV1PeriodUpdateDto {
            id,
            author,
            publish,
            publish_locations,
            translations,
            created_at,
            updated_at,
        }
    }
}

/// (Legal values: riotclient, riotstatus, game)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PublishLocations {}
