//! Alerting system for security events
//!
//! This module implements a simple alerting mechanism to notify administrators
//! of security events and anomalies detected in the system.

use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a security alert
#[derive(Debug, Clone, Serialize)]
pub struct SecurityAlert {
    pub alert_type: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: u64,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Alert manager for handling security alerts
pub struct AlertManager {
    alerts: Arc<Mutex<Vec<SecurityAlert>>>,
}

impl AlertManager {
    /// Create a new alert manager
    pub fn new() -> Self {
        Self {
            alerts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Generate an alert for anomalous user behavior
    pub fn alert_user_anomaly(&self, user_id: &str, attempt_count: u32) {
        let alert = SecurityAlert {
            alert_type: "USER_ANOMALY".to_string(),
            message: format!(
                "User {} shows anomalous behavior with {} attempts in the time window",
                user_id, attempt_count
            ),
            severity: if attempt_count > 20 {
                AlertSeverity::High
            } else {
                AlertSeverity::Medium
            },
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            user_id: Some(user_id.to_string()),
            ip_address: None,
        };

        self.record_alert(alert);
    }

    /// Generate an alert for anomalous IP behavior
    pub fn alert_ip_anomaly(&self, ip_address: &str, attempt_count: u32, failed_attempts: u32) {
        let alert = SecurityAlert {
            alert_type: "IP_ANOMALY".to_string(),
            message: format!(
                "IP {} shows anomalous behavior with {} total attempts and {} failed attempts",
                ip_address, attempt_count, failed_attempts
            ),
            severity: if failed_attempts > 10 {
                AlertSeverity::High
            } else {
                AlertSeverity::Medium
            },
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            user_id: None,
            ip_address: Some(ip_address.to_string()),
        };

        self.record_alert(alert);
    }

    /// Generate an alert for brute force attack detection
    pub fn alert_brute_force(&self, user_id: &str, ip_address: &str, failed_attempts: u32) {
        let alert = SecurityAlert {
            alert_type: "BRUTE_FORCE".to_string(),
            message: format!("Potential brute force attack detected from IP {} on user {} with {} failed attempts", 
                           ip_address, user_id, failed_attempts),
            severity: AlertSeverity::Critical,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            user_id: Some(user_id.to_string()),
            ip_address: Some(ip_address.to_string()),
        };

        self.record_alert(alert);
    }

    /// Record an alert
    fn record_alert(&self, alert: SecurityAlert) {
        let mut alerts = self.alerts.lock().unwrap();
        alerts.push(alert);

        // Keep only the last 100 alerts to prevent memory issues
        let len = alerts.len();
        if len > 100 {
            alerts.drain(0..len - 100);
        }

        // Print to console (in a real system, this would send to a monitoring system)
        if let Some(latest) = alerts.last() {
            println!(
                "SECURITY ALERT: {:?} - {}",
                latest.severity, latest.message
            );
        }
    }

    /// Get recent alerts
    pub fn get_recent_alerts(&self, count: usize) -> Vec<SecurityAlert> {
        let alerts = self.alerts.lock().unwrap();
        let start = if alerts.len() > count {
            alerts.len() - count
        } else {
            0
        };
        alerts[start..].to_vec()
    }

    /// Get alerts by type
    pub fn get_alerts_by_type(&self, alert_type: &str) -> Vec<SecurityAlert> {
        let alerts = self.alerts.lock().unwrap();
        alerts
            .iter()
            .filter(|alert| alert.alert_type == alert_type)
            .cloned()
            .collect()
    }

    /// Get alerts by severity
    pub fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Vec<SecurityAlert> {
        let alerts = self.alerts.lock().unwrap();
        alerts
            .iter()
            .filter(|alert| alert.severity == severity)
            .cloned()
            .collect()
    }

    /// Clear all alerts
    pub fn clear_alerts(&self) {
        let mut alerts = self.alerts.lock().unwrap();
        alerts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_creation() {
        let alert_manager = AlertManager::new();

        alert_manager.alert_user_anomaly("user123", 15);
        alert_manager.alert_ip_anomaly("192.168.1.1", 25, 12);
        alert_manager.alert_brute_force("user456", "10.0.0.1", 20);

        let recent_alerts = alert_manager.get_recent_alerts(10);
        assert_eq!(recent_alerts.len(), 3);

        let user_alerts = alert_manager.get_alerts_by_type("USER_ANOMALY");
        assert_eq!(user_alerts.len(), 1);

        let critical_alerts = alert_manager.get_alerts_by_severity(AlertSeverity::Critical);
        assert_eq!(critical_alerts.len(), 1);
    }
}
