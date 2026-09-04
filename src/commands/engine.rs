use crate::core::config::{self, Config};

pub fn run(config: &Config, args: &[String]) {
    if args.is_empty() {
        list(config);
        return;
    }

    match args[0].as_str() {
        "list" | "ls" => list(config),

        "info" => {
            if args.len() < 2 {
                println!("Usage: quarry engine info <name>");
                return;
            }

            info(config, &args[1]);
        }

        "add" => add(&args[1..]),

        "remove" | "rm" => remove(config, &args[1..]),

        _ => {
            println!("Unknown engine command: {}", args[0]);
        }
    }
}

pub fn list(config: &Config) {
    println!("Search engines:");

    for (name, engine) in &config.search_engines {
        println!("  {:<5} {}", engine.shortcut, name);
    }
}

pub fn info(config: &Config, name: &str) {
    let Some((name, engine)) = config.search_engines
        .iter()
        .find(|(engine_name, engine)| {
            engine_name.as_str() == name
                || engine.shortcut == name
        })
    else {
        println!("Unknown search engine: {}", name);
        return;
    };

    println!("Name:      {}", name);
    println!("Shortcut:  {}", engine.shortcut);
    println!("URL:       {}", engine.url);
}

fn add(args: &[String]) {
    if args.len() < 3 {
        println!(
            "Usage: quarry engine add <name> <shortcut> <url>"
        );
        return;
    }

    let name = &args[0];
    let shortcut = &args[1];
    let url = &args[2];

    config::add_search_engine(name, shortcut, url);

    println!("Search engine '{}' added.", name);
}

fn remove(config: &Config, args: &[String]) {
    if args.is_empty() {
        println!("Usage: quarry engine remove <name>");
        return;
    }

    let name = &args[0];

    if !config.search_engines.contains_key(name) {
        println!("Unknown search engine: {}", name);
        return;
    }

    config::remove_search_engine(name);

    println!("Search engine '{}' removed.", name);
}
