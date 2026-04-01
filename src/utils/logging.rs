//! Logging setup.

/// Initializes the logger for the native host.
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}
