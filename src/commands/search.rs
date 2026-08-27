use std::process::{Command, Stdio};

use crate::core::config::Config;

pub fn run(config: &Config, command: &str, args: &[String]) {
    let Some((_name, engine)) = config.search_engines
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

fn open_browser(config: &Config, url: &str) {
    Command::new(&config.browser)
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to open browser");
}
