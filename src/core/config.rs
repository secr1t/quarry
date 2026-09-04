use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &str = include_str!("../../assets/config.toml");

#[derive(Deserialize, Clone)]
pub struct Config {
    pub browser: String,

    #[serde(default)]
    pub search_engines: HashMap<String, SearchEngine>,

    #[serde(default)]
    pub favorites: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SearchEngine {
    pub shortcut: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Default)]
struct UserConfig {
    browser: Option<String>,

    search_engines: Option<HashMap<String, SearchEngine>>,

    favorites: Option<HashMap<String, String>>,

    #[serde(default)]
    removed_favorites: Vec<String>,

    #[serde(default)]
    removed_search_engines: Vec<String>,
}

pub fn load_config() -> Config {
    let mut config: Config = toml::from_str(DEFAULT_CONFIG)
        .expect("Failed to parse default config");

    let path = config_path();

    if let Ok(text) = fs::read_to_string(&path) {
        let user_config: UserConfig = toml::from_str(&text)
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

        for name in user_config.removed_favorites {
            config.favorites.remove(&name);
        }

        for name in user_config.removed_search_engines {
            config.search_engines.remove(&name);
        }
    }

    config
}

fn load_user_config() -> UserConfig {
    let path = config_path();

    match fs::read_to_string(&path) {
        Ok(text) => toml::from_str(&text)
            .expect("Failed to parse user config"),

        Err(_) => UserConfig::default(),
    }
}

fn save_user_config(config: &UserConfig) {
    let path = config_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .expect("Failed to create config directory");
    }

    let text = toml::to_string_pretty(config)
        .expect("Failed to serialize user config");

    fs::write(path, text)
        .expect("Failed to save user config");
}

pub fn add_favorite(name: &str, url: &str) {
    let mut config = load_user_config();

    let favorites = config.favorites
        .get_or_insert_with(HashMap::new);

    favorites.insert(name.to_string(), url.to_string());

    config.removed_favorites.retain(|item| item != name);

    save_user_config(&config);
}

pub fn remove_favorite(name: &str) {
    let mut config = load_user_config();

    if let Some(favorites) = &mut config.favorites {
        favorites.remove(name);
    }

    if !config.removed_favorites.iter().any(|item| item == name) {
        config.removed_favorites.push(name.to_string());
    }

    save_user_config(&config);
}

pub fn add_search_engine(
    name: &str,
    shortcut: &str,
    url: &str,
) {
    let mut config = load_user_config();

    let engines = config.search_engines
        .get_or_insert_with(HashMap::new);

    engines.insert(
        name.to_string(),
        SearchEngine {
            shortcut: shortcut.to_string(),
            url: url.to_string(),
        },
    );

    config.removed_search_engines
        .retain(|item| item != name);

    save_user_config(&config);
}

pub fn remove_search_engine(name: &str) {
    let mut config = load_user_config();

    if let Some(engines) = &mut config.search_engines {
        engines.remove(name);
    }

    if !config.removed_search_engines.iter().any(|item| item == name) {
        config.removed_search_engines.push(name.to_string());
    }

    save_user_config(&config);
}

pub fn config_path() -> PathBuf {
    let config_home = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(
                env::var("HOME")
                    .expect("HOME environment variable is not set"),
            )
            .join(".config")
        });

    config_home.join("quarry").join("config.toml")
}
