




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV3PeriodShardStatus {
    #[serde(rename = "locales")]
    pub locales: Vec<String>,
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "services")]
    pub services: Vec<crate::models::LolStatusV3PeriodService>,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "region_tag")]
    pub region_tag: String,
}

impl LolStatusV3PeriodShardStatus {
    pub fn new(locales: Vec<String>, hostname: String, name: String, services: Vec<crate::models::LolStatusV3PeriodService>, slug: String, region_tag: String) -> LolStatusV3PeriodShardStatus {
        LolStatusV3PeriodShardStatus {
            locales,
            hostname,
            name,
            services,
            slug,
            region_tag,
        }
    }
}


