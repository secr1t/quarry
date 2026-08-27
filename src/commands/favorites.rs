use crate::core::config::{self, Config};

use super::engine;

pub fn run(config: &Config, args: &[String]) {
    if args.is_empty() {
        list(config);
        return;
    }

    let name = &args[0];

    let Some(url) = config.favorites.get(name) else {
        println!("Unknown favorite: {}", name);
        return;
    };

    engine::open_url(config, url);
}

pub fn add(args: &[String]) {
    if args.len() < 2 {
        println!("Usage: quarry add favorite <name> <url>");
        return;
    }

    let name = &args[0];
    let url = &args[1];

    config::add_favorite(name, url);

    println!("Favorite '{}' added.", name);
}

pub fn remove(args: &[String]) {
    if args.is_empty() {
        println!("Usage: quarry remove favorite <name>");
        return;
    }

    let name = &args[0];

    config::remove_favorite(name);

    println!("Favorite '{}' removed.", name);
}

fn list(config: &Config) {
    println!("Favorites:");

    for (name, url) in &config.favorites {
        println!("  {:<12} {}", name, url);
    }
}
