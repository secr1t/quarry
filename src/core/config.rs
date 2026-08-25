use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../../assets/config.toml");

#[derive(Deserialize, Clone)]
pub struct Config {
    pub browser: String,
    pub search_engines: HashMap<String, SearchEngine>,
    pub favorites: HashMap<String, String>,
}

#[derive(Deserialize, Clone)]
pub struct SearchEngine {
    pub shortcut: String,
    pub url: String,
}

#[derive(Deserialize, Default)]
struct UserConfig {
    browser: Option<String>,
    search_engines: Option<HashMap<String, SearchEngine>>,
    favorites: Option<HashMap<String, String>>,
}

pub fn load_config() -> Config {
    let mut config: Config =
        toml::from_str(DEFAULT_CONFIG)
            .expect("Failed to parse default config");

    let path = config_path();

    if let Ok(text) = fs::read_to_string(path) {
        let user_config: UserConfig =
            toml::from_str(&text)
                .expect("Failed to parse user config");

        if let Some(browser) = user_config.browser {
            config.browser = browser;
        }

        if let Some(search_engines) = user_config.search_engines {
            config.search_engines.extend(search_engines);
        }

        if let Some(favorites) = user_config.favorites {
            config.favorites.extend(favorites);
        }
    }

    config
}

fn config_path() -> PathBuf {
    let config_home = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(
                env::var("HOME")
                    .expect("HOME environment variable is not set")
            )
            .join(".config")
        });

    config_home.join("quarry").join("config.toml")
}
