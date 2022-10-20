#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorStatus {
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ErrorStatus {
    pub fn new() -> ErrorStatus {
        ErrorStatus {
            status_code: None,
            message: None,
        }
    }
}
