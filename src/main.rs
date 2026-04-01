//! VZGLYD Native Binary
//!
//! Main entry point for the native host.

use std::process::ExitCode;
use vzglyd_native::NativeApp;

fn main() -> ExitCode {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("VZGLYD Native Host starting...");

    // Run the application
    match NativeApp::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            log::error!("Application error: {}", e);
            ExitCode::FAILURE
        }
    }
}
