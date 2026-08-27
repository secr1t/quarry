use crate::core::config::Config;

pub mod engine;
pub mod favorites;
pub mod help;
pub mod list;

pub fn run(config: &Config, args: &[String]) {
    if args.is_empty() {
        println!("No command specified. Use 'quarry help' for help.");
        return;
    }

    let command = &args[0];

    match command.as_str() {
        "help" => help::run(config),

        "list" => list::run(config),

        "favorites" | "f" => {
            favorites::run(config, &args[1..]);
        }

        "add" => {
            handle_add(&args[1..]);
        }

        "remove" => {
            handle_remove(&args[1..]);
        }

        _ => {
            engine::run(config, command, &args[1..]);
        }
    }
}

fn handle_add(args: &[String]) {
    if args.is_empty() {
        println!("Usage: quarry add favorite <name> <url>");
        return;
    }

    match args[0].as_str() {
        "favorite" | "f" => favorites::add(&args[1..]),

        _ => {
            println!("Unknown add target: {}", args[0]);
        }
    }
}

fn handle_remove(args: &[String]) {
    if args.is_empty() {
        println!("Usage: quarry remove favorite <name>");
        return;
    }

    match args[0].as_str() {
        "favorite" | "f" => favorites::remove(&args[1..]),

        _ => {
            println!("Unknown remove target: {}", args[0]);
        }
    }
}
