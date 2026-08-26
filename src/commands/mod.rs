use std::process::{Command, Stdio};

use crate::core::config::Config;

pub mod help;

pub fn run(config: &Config, args: &[String]) {
    if args.is_empty() {
        println!("No command specified. Use 'quarry help' for help.");
        return;
    }

    let command = &args[0];

    match command.as_str() {
        "help" => help::run(config),

        "list" => list(config),

        "favorites" | "f" => favorites(config, &args[1..]),

        _ => search(config, command, &args[1..]),
    }
}

fn search(config: &Config, command: &str, args: &[String]) {
    let Some((name, engine)) = config.search_engines
        .iter()
        .find(|(name, engine)| {
            name.as_str() == command || engine.shortcut == command
        })
    else {
        println!("Unknown command: {}", command);
        return;
    };

    if args.is_empty() {
        println!("Query is missing.");
        return;
    }

    let query = args.join(" ");
    let url = engine.url.replace("{query}", &query);

    open_browser(config, &url);
}

fn favorites(config: &Config, args: &[String]) {
    if args.is_empty() {
        println!("Favorites:");

        for name in config.favorites.keys() {
            println!("  {}", name);
        }

        return;
    }

    let name = &args[0];

    let Some(url) = config.favorites.get(name) else {
        println!("Unknown favorite: {}", name);
        return;
    };

    open_browser(config, url);
}

fn list(config: &Config) {
    println!("Search engines:");

    for (name, engine) in &config.search_engines {
        println!("  {:<5} {}", engine.shortcut, name);
    }

    println!();
    println!("Favorites:");

    for name in config.favorites.keys() {
        println!("  {}", name);
    }
}

fn open_browser(config: &Config, url: &str) {
    Command::new(&config.browser)
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to open browser");
}
