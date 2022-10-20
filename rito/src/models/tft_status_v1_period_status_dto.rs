

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TftStatusV1PeriodStatusDto {
    #[serde(rename = "id")]
    pub id: i32,
    /// (Legal values:  scheduled,  in_progress,  complete)
    #[serde(rename = "maintenance_status")]
    pub maintenance_status: MaintenanceStatus,
    /// (Legal values:  info,  warning,  critical)
    #[serde(rename = "incident_severity")]
    pub incident_severity: IncidentSeverity,
    #[serde(rename = "titles")]
    pub titles: Vec<crate::models::TftStatusV1PeriodContentDto>,
    #[serde(rename = "updates")]
    pub updates: Vec<crate::models::TftStatusV1PeriodUpdateDto>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "archive_at")]
    pub archive_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// (Legal values: windows, macos, android, ios, ps4, xbone, switch)
    #[serde(rename = "platforms")]
    pub platforms: Vec<Platforms>,
}

impl TftStatusV1PeriodStatusDto {
    pub fn new(
        id: i32,
        maintenance_status: MaintenanceStatus,
        incident_severity: IncidentSeverity,
        titles: Vec<crate::models::TftStatusV1PeriodContentDto>,
        updates: Vec<crate::models::TftStatusV1PeriodUpdateDto>,
        created_at: String,
        archive_at: String,
        updated_at: String,
        platforms: Vec<Platforms>,
    ) -> TftStatusV1PeriodStatusDto {
        TftStatusV1PeriodStatusDto {
            id,
            maintenance_status,
            incident_severity,
            titles,
            updates,
            created_at,
            archive_at,
            updated_at,
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
