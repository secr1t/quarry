use crate::core::config::Config;

pub fn run(config: &Config) {
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
