use std::collections::HashMap;

use serde::Deserialize;

use crate::core::config::Config;

const HELP: &str = include_str!("../../assets/help.toml");

#[derive(Deserialize)]
struct Help {
    common: String,
    description: String,
    commands: HashMap<String, CommandHelp>,
}

#[derive(Deserialize)]
struct CommandHelp {
    description: String,
}

pub fn run(config: &Config) {
    let help: Help =
        toml::from_str(HELP)
            .expect("Failed to parse help");

    println!("{}", help.common);
    println!();
    println!("{}", help.description);
    println!();

    println!("Search engines:");

    for (name, engine) in &config.search_engines {
        println!(
            "  {:<5} {:<12} {}",
            engine.shortcut,
            name,
            engine.url
        );
    }

    println!();

    println!("Commands:");

    for (name, command) in &help.commands {
        println!(
            "  {:<12} {}",
            name,
            command.description
        );
    }
}
