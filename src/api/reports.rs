//! Reports API
//!
//! This module provides functionality to get tenant reports.

use crate::client::RainClient;
use crate::error::Result;
use crate::models::reports::*;

impl RainClient {
    /// Get a tenant's report
    ///
    /// # Arguments
    ///
    /// * `year` - Year of the report
    /// * `month` - Month of the report
    /// * `day` - Day of the report
    /// * `params` - Optional query parameters (format)
    ///
    /// # Returns
    ///
    /// Returns the report as raw bytes (content type depends on format: csv, json, or ssrp).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rain_sdk::{RainClient, Config, Environment, AuthConfig};
    /// use rain_sdk::models::reports::{GetReportParams, ReportFormat};
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Dev);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = RainClient::new(config, auth)?;
    ///
    /// let params = GetReportParams {
    ///     format: Some(ReportFormat::Csv),
    /// };
    /// let report = client.get_report("2024", "01", "15", &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_report(
        &self,
        year: &str,
        month: &str,
        day: &str,
        params: &GetReportParams,
    ) -> Result<Vec<u8>> {
        let base_url = self.base_url();
        let mut url = base_url.clone();
        url.path_segments_mut()
            .map_err(|_| crate::error::RainError::Other(anyhow::anyhow!("Cannot be a base URL")))?
            .pop_if_empty()
            .push("issuing")
            .push("reports")
            .push(year)
            .push(month)
            .push(day);

        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            format!("/issuing/reports/{year}/{month}/{day}")
        } else {
            format!("/issuing/reports/{year}/{month}/{day}?{query_string}")
        };
        self.get_bytes(&full_path).await
    }

    // ============================================================================
    // Blocking Methods
    // ============================================================================

    /// Get a tenant's report (blocking)
    #[cfg(feature = "sync")]
    pub fn get_report_blocking(
        &self,
        year: &str,
        month: &str,
        day: &str,
        params: &GetReportParams,
    ) -> Result<Vec<u8>> {
        let query_string = serde_urlencoded::to_string(params)?;
        let full_path = if query_string.is_empty() {
            format!("/issuing/reports/{year}/{month}/{day}")
        } else {
            format!("/issuing/reports/{year}/{month}/{day}?{query_string}")
        };
        self.get_bytes_blocking(&full_path)
    }
}
