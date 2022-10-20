




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LorDeckV1PeriodNewDeckDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "code")]
    pub code: String,
}

impl LorDeckV1PeriodNewDeckDto {
    pub fn new(name: String, code: String) -> LorDeckV1PeriodNewDeckDto {
        LorDeckV1PeriodNewDeckDto {
            name,
            code,
        }
    }
}


