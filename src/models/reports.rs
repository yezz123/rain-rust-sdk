//! Models for report endpoints

use serde::{Deserialize, Serialize};

/// Format of the report
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReportFormat {
    Csv,
    Json,
    Ssrp,
}

/// Query parameters for getting a report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReportParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ReportFormat>,
}

/// Report response (raw bytes, content type depends on format)
pub type ReportResponse = Vec<u8>;
