use crate::core::config::{self, Config};

use super::search;

pub fn run(config: &Config, args: &[String]) {
    if args.is_empty() {
        list(config);
        return;
    }

    match args[0].as_str() {
        "list" | "ls" => list(config),

        "add" => add(&args[1..]),

        "remove" | "rm" => remove(config, &args[1..]),

        name => open(config, name),
    }
}

pub fn list(config: &Config) {
    println!("Favorites:");

    for (name, url) in &config.favorites {
        println!("  {:<12} {}", name, url);
    }
}

fn open(config: &Config, name: &str) {
    let Some(url) = config.favorites.get(name) else {
        println!("Unknown favorite: {}", name);
        return;
    };

    search::open_browser(config, url);
}

fn add(args: &[String]) {
    if args.len() < 2 {
        println!(
            "Usage: quarry favorites add <name> <url>"
        );
        return;
    }

    let name = &args[0];
    let url = &args[1];

    config::add_favorite(name, url);

    println!("Favorite '{}' added.", name);
}

fn remove(config: &Config, args: &[String]) {
    if args.is_empty() {
        println!(
            "Usage: quarry favorites remove <name>"
        );
        return;
    }

    let name = &args[0];

    if !config.favorites.contains_key(name) {
        println!("Unknown favorite: {}", name);
        return;
    }

    config::remove_favorite(name);

    println!("Favorite '{}' removed.", name);
}
