use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub discord_token: String,
    pub twitch_client_id: String,
    pub twitch_oauth: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_str = fs::read_to_string("config.toml").expect("Failed to read config");
    toml::from_str(&config_str).expect("Failed to parse config")
});

pub fn get_config() -> &'static Config {
    &CONFIG
}
