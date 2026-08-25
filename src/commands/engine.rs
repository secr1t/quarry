pub fn run(args: &[String]) {
    let command = &args[0];

    match command.as_str() {
        "help" => help::run(&args[1..]),
        "favorites" => favorites::run(&args[1..]),

        _ => {
            if let Some(engine) = find_engine(&config, command) {
                search(engine, &args[1..]);
            } else {
                println!("Unknown command: {}", command);
            }
        }
    }
}
