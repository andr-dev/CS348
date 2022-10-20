




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::ErrorStatus>>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            status: None,
        }
    }
}


