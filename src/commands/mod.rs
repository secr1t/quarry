use crate::core::config::Config;

pub mod engine;
pub mod favorites;
pub mod help;
pub mod list;
pub mod search;

pub fn run(config: &Config, args: &[String]) {
    if args.is_empty() {
        println!("No command specified. Use 'quarry help' for help.");
        return;
    }

    let command = &args[0];

    match command.as_str() {
        "help" => {
            help::run(config);
        }

        "list" => {
            list::run(config);
        }

        "favorites" | "f" => {
            favorites::run(config, &args[1..]);
        }

        "engine" | "engines" | "e" => {
            engine::run(config, &args[1..]);
        }

        _ => {
            search::run(config, command, &args[1..]);
        }
    }
}
