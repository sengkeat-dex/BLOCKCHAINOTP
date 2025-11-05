//! Anomaly detection for OTP verification attempts
//!
//! This module implements detection of suspicious patterns in OTP verification
//! to identify potential brute force attacks or other malicious activities.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents an OTP verification attempt for anomaly detection
#[derive(Debug, Clone)]
pub struct VerificationAttempt {
    pub user_id: String,
    pub ip_address: String,
    pub timestamp: u64,
    pub success: bool,
}

/// Anomaly detection statistics for a user/IP
#[derive(Debug, Clone)]
pub struct AnomalyStats {
    pub attempt_count: u32,
    pub failed_attempts: u32,
    pub last_attempt_timestamp: u64,
    pub first_attempt_timestamp: u64,
}

/// Anomaly detection engine
pub struct AnomalyDetector {
    /// Statistics per user ID
    user_stats: Arc<Mutex<HashMap<String, AnomalyStats>>>,
    /// Statistics per IP address
    ip_stats: Arc<Mutex<HashMap<String, AnomalyStats>>>,
    /// Time window for anomaly detection (in seconds)
    time_window: u64,
    /// Threshold for flagging anomalies
    threshold: u32,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new(time_window: u64, threshold: u32) -> Self {
        Self {
            user_stats: Arc::new(Mutex::new(HashMap::new())),
            ip_stats: Arc::new(Mutex::new(HashMap::new())),
            time_window,
            threshold,
        }
    }

    /// Record a verification attempt
    pub fn record_attempt(&self, attempt: VerificationAttempt) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Update user statistics
        {
            let mut user_stats = self.user_stats.lock().unwrap();
            let stats = user_stats
                .entry(attempt.user_id.clone())
                .or_insert_with(|| AnomalyStats {
                    attempt_count: 0,
                    failed_attempts: 0,
                    last_attempt_timestamp: now,
                    first_attempt_timestamp: now,
                });

            stats.attempt_count += 1;
            if !attempt.success {
                stats.failed_attempts += 1;
            }
            stats.last_attempt_timestamp = now;

            // Clean up old entries outside the time window
            user_stats.retain(|_, s| now - s.first_attempt_timestamp <= self.time_window);
        }

        // Update IP statistics
        {
            let mut ip_stats = self.ip_stats.lock().unwrap();
            let stats = ip_stats
                .entry(attempt.ip_address.clone())
                .or_insert_with(|| AnomalyStats {
                    attempt_count: 0,
                    failed_attempts: 0,
                    last_attempt_timestamp: now,
                    first_attempt_timestamp: now,
                });

            stats.attempt_count += 1;
            if !attempt.success {
                stats.failed_attempts += 1;
            }
            stats.last_attempt_timestamp = now;

            // Clean up old entries outside the time window
            ip_stats.retain(|_, s| now - s.first_attempt_timestamp <= self.time_window);
        }
    }

    /// Check if a user shows anomalous behavior
    pub fn is_user_anomalous(&self, user_id: &str) -> bool {
        let user_stats = self.user_stats.lock().unwrap();
        if let Some(stats) = user_stats.get(user_id) {
            stats.attempt_count > self.threshold
        } else {
            false
        }
    }

    /// Check if an IP shows anomalous behavior
    pub fn is_ip_anomalous(&self, ip_address: &str) -> bool {
        let ip_stats = self.ip_stats.lock().unwrap();
        if let Some(stats) = ip_stats.get(ip_address) {
            stats.attempt_count > self.threshold || stats.failed_attempts > self.threshold / 2
        } else {
            false
        }
    }

    /// Get statistics for a user
    pub fn get_user_stats(&self, user_id: &str) -> Option<AnomalyStats> {
        let user_stats = self.user_stats.lock().unwrap();
        user_stats.get(user_id).cloned()
    }

    /// Get statistics for an IP
    pub fn get_ip_stats(&self, ip_address: &str) -> Option<AnomalyStats> {
        let ip_stats = self.ip_stats.lock().unwrap();
        ip_stats.get(ip_address).cloned()
    }

    /// Reset statistics for a user
    pub fn reset_user_stats(&self, user_id: &str) {
        let mut user_stats = self.user_stats.lock().unwrap();
        user_stats.remove(user_id);
    }

    /// Reset statistics for an IP
    pub fn reset_ip_stats(&self, ip_address: &str) {
        let mut ip_stats = self.ip_stats.lock().unwrap();
        ip_stats.remove(ip_address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_detection() {
        let detector = AnomalyDetector::new(3600, 5); // 1 hour window, 5 attempts threshold

        // Record normal attempts
        for i in 0..3 {
            let attempt = VerificationAttempt {
                user_id: "user123".to_string(),
                ip_address: "192.168.1.1".to_string(),
                timestamp: 1000 + i,
                success: true,
            };
            detector.record_attempt(attempt);
        }

        // Should not be anomalous yet
        assert!(!detector.is_user_anomalous("user123"));
        assert!(!detector.is_ip_anomalous("192.168.1.1"));

        // Record more attempts to exceed threshold
        for i in 0..3 {
            let attempt = VerificationAttempt {
                user_id: "user123".to_string(),
                ip_address: "192.168.1.1".to_string(),
                timestamp: 2000 + i,
                success: false,
            };
            detector.record_attempt(attempt);
        }

        // Should now be anomalous
        assert!(detector.is_user_anomalous("user123"));
        assert!(detector.is_ip_anomalous("192.168.1.1"));
    }

    #[test]
    fn test_stats_retrieval() {
        let detector = AnomalyDetector::new(3600, 5);

        let attempt = VerificationAttempt {
            user_id: "user456".to_string(),
            ip_address: "10.0.0.1".to_string(),
            timestamp: 1000,
            success: true,
        };
        detector.record_attempt(attempt);

        let user_stats = detector.get_user_stats("user456");
        assert!(user_stats.is_some());
        let user_stats = user_stats.unwrap();
        assert_eq!(user_stats.attempt_count, 1);

        let ip_stats = detector.get_ip_stats("10.0.0.1");
        assert!(ip_stats.is_some());
        let ip_stats = ip_stats.unwrap();
        assert_eq!(ip_stats.attempt_count, 1);
    }
}
