

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LolStatusV4PeriodStatusDto {
    #[serde(rename = "id")]
    pub id: i32,
    /// (Legal values:  scheduled,  in_progress,  complete)
    #[serde(rename = "maintenance_status", skip_serializing_if = "Option::is_none")]
    pub maintenance_status: Option<MaintenanceStatus>,
    /// (Legal values:  info,  warning,  critical)
    #[serde(rename = "incident_severity", skip_serializing_if = "Option::is_none")]
    pub incident_severity: Option<IncidentSeverity>,
    #[serde(rename = "titles")]
    pub titles: Vec<crate::models::LolStatusV4PeriodContentDto>,
    #[serde(rename = "updates")]
    pub updates: Vec<crate::models::LolStatusV4PeriodUpdateDto>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "archive_at", skip_serializing_if = "Option::is_none")]
    pub archive_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// (Legal values: windows, macos, android, ios, ps4, xbone, switch)
    #[serde(rename = "platforms")]
    pub platforms: Vec<Platforms>,
}

impl LolStatusV4PeriodStatusDto {
    pub fn new(
        id: i32,
        titles: Vec<crate::models::LolStatusV4PeriodContentDto>,
        updates: Vec<crate::models::LolStatusV4PeriodUpdateDto>,
        created_at: String,
        platforms: Vec<Platforms>,
    ) -> LolStatusV4PeriodStatusDto {
        LolStatusV4PeriodStatusDto {
            id,
            maintenance_status: None,
            incident_severity: None,
            titles,
            updates,
            created_at,
            archive_at: None,
            updated_at: None,
            platforms,
        }
    }
}

/// (Legal values:  scheduled,  in_progress,  complete)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MaintenanceStatus {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "complete")]
    Complete,
}

impl Default for MaintenanceStatus {
    fn default() -> MaintenanceStatus {
        Self::Scheduled
    }
}
/// (Legal values:  info,  warning,  critical)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentSeverity {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "critical")]
    Critical,
}

impl Default for IncidentSeverity {
    fn default() -> IncidentSeverity {
        Self::Info
    }
}
/// (Legal values: windows, macos, android, ios, ps4, xbone, switch)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Platforms {}
