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

    #[serde(default)]
    subcommands: HashMap<String, CommandHelp>,
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
    print_commands("Commands:", &help.commands, 2);
}

fn print_commands(
    title: &str,
    commands: &HashMap<String, CommandHelp>,
    indent: usize,
) {
    println!("{}", title);

    for (name, command) in commands {
        let prefix = " ".repeat(indent);
        println!(
            "{} {:<24} {}",
            prefix,
            name,
            command.description
        );

        if !command.subcommands.is_empty() {
            print_commands("Subcommands:", &command.subcommands, indent + 2);
        }
    }
}
