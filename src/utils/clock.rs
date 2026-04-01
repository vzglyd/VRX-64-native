//! Clock and time utilities.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time in seconds since UNIX epoch.
pub fn now_secs() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}
