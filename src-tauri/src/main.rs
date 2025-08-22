// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    // Initialize tracing with env-filter
    tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive("byte_me=debug".parse().unwrap())
                .add_directive("tauri=info".parse().unwrap()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting byte-me application");
    byte_me_lib::run()
}
