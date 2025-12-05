use crate::conf::app::AppConfig;
use std::sync::LazyLock;

pub mod app;
mod base;
mod database;
mod redis;

// set the static config
static APP_CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initial app config"));

// get the static config pointer
pub fn get_app_config() -> &'static AppConfig {
    &APP_CONFIG
}
