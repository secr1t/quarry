# Quarry

Quarry is a lightweight CLI utility for searching the web and opening favorite websites from the terminal.

It provides a simple interface for multiple search engines through configurable names and shortcuts, with bundled defaults and a user configuration layer.

## Features

- Search multiple websites from one command.
- Use full engine names or short shortcuts.
- Add, inspect, list, and remove search engines.
- Open, add, list, and remove favorite websites.
- Configure search engines and favorites with TOML.
- Built-in default configuration with optional user overrides.
- Use the system default browser or configure a specific browser.
- Available through both `quarry` and the shorter `q` binary.

## Usage

### Search

Search using a full engine name:

```bash
quarry youtube "hollow knight"
quarry google "rust wgpu"
```

Search using a shortcut:

```bash
quarry y "hollow knight"
quarry g "rust wgpu"
```

The same commands are available through the short binary:

```bash
q youtube "hollow knight"
q y "hollow knight"
```

### Search engines

List configured search engines:

```bash
quarry engine list
```

Inspect an engine by name or shortcut:

```bash
quarry engine info youtube
quarry engine info y
```

Add a search engine:

```bash
quarry engine add stackoverflow so "https://stackoverflow.com/search?q={query}"
```

Remove a search engine by name or shortcut:

```bash
quarry engine remove stackoverflow
quarry engine remove so
```

The URL must contain `{query}`, which Quarry replaces with the search query.

### Favorites

List favorites:

```bash
quarry favorites list
```

Open a favorite:

```bash
quarry favorites github
```

Add a favorite:

```bash
quarry favorites add gitlab "https://gitlab.com"
```

Remove a favorite:

```bash
quarry favorites remove gitlab
```

### Other commands

Show help:

```bash
quarry help
```

List both search engines and favorites:

```bash
quarry list
```

## Configuration

Quarry uses TOML for configuration.

The default configuration is bundled into the binary at compile time. User configuration can override or extend it without modifying the built-in defaults.

On Linux, the user configuration is loaded from:

```text
~/.config/quarry/config.toml
```

or from `$XDG_CONFIG_HOME/quarry/config.toml` when `XDG_CONFIG_HOME` is set.

Quarry creates the configuration directory and file automatically when a command modifies the user configuration.

### Browser

The default configuration uses `xdg-open`, which delegates URL handling to the system's default application:

```toml
browser = "xdg-open"
```

A specific browser can be configured instead:

```toml
browser = "firefox"
```

### Search engines

A search engine is defined by a name, a shortcut, and a URL template:

```toml
[search_engines.youtube]
shortcut = "y"
url = "https://www.youtube.com/results?search_query={query}"
```

This makes both of the following commands available:

```bash
quarry youtube "hollow knight"
quarry y "hollow knight"
```

### Favorites

Favorites are simple name-to-URL mappings:

```toml
[favorites]
github = "https://github.com"
arch = "https://wiki.archlinux.org"
```

### User configuration and defaults

The built-in configuration is never modified by Quarry. User settings are stored separately and merged with the defaults when Quarry starts.

For example, adding a custom search engine creates an entry in the user configuration:

```toml
[search_engines.stackoverflow]
shortcut = "so"
url = "https://stackoverflow.com/search?q={query}"
```

Removing a built-in search engine records that removal in the user configuration rather than changing the bundled defaults.

## Installation

Build and install Quarry with Cargo:

```bash
cargo install --path .
```

The package provides two binaries:

```text
quarry
q
```

Both use the same underlying code and configuration.

## Development

Clone the repository:

```bash
git clone https://github.com/secr1t/quarry.git
cd quarry
```

Run Quarry directly with Cargo:

```bash
cargo run -- help
cargo run -- y "hollow knight"
```

Build the release binary:

```bash
cargo build --release
```

## Project structure

```text
quarry/
├── assets/
│   ├── config.toml
│   └── help.toml
├── src/
│   ├── bin/
│   │   └── q.rs
│   ├── commands/
│   │   ├── engine.rs
│   │   ├── favorites.rs
│   │   ├── help.rs
│   │   ├── list.rs
│   │   ├── mod.rs
│   │   └── search.rs
│   ├── core/
│   │   ├── config.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
└── Cargo.toml
```

## License

Quarry is free software licensed under the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

See [LICENSE](LICENSE) for the full license text.
