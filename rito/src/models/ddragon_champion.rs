

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DDragonChampionResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, DDragonChampion>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DDragonChampion {
    pub name: String,
    pub key: String,
    pub title: String,
    pub blurb: String,
}

impl DDragonChampion {
    pub fn new(name: String, key: String, title: String, blurb: String) -> Self {
        Self {
            name,
            key,
            title,
            blurb,
        }
    }
}
