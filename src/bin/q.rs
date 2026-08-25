use quarry::{commands, core};

fn main() {
    let config = core::config::load_config();
    let args: Vec<String> = std::env::args().skip(1).collect();

    commands::run(&config, &args);
}
