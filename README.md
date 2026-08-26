# Quarry

Quarry is a small CLI utility for searching the web and opening favorite websites from the terminal.

It provides a simple interface for multiple search engines through configurable shortcuts, while keeping the default configuration bundled with the application.

## Features

- Search multiple websites from one command.
- Use full engine names or short shortcuts.
- Open favorite websites.
- Configure everything through TOML.
- Built-in default configuration with optional user overrides.
- Available through both `quarry` and the shorter `q` binary.
- Choose the browser used to open search results.

## Usage

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

Display help:

```bash
quarry help
q help
```

List available search engines and favorites:

```bash
quarry list
```

## Configuration

Quarry uses TOML for configuration.

The built-in default configuration is bundled into the binary at compile time. A user configuration can override or extend it without modifying the built-in defaults.

On Linux, the user configuration is loaded from:

```text
~/.config/quarry/config.toml
```

or from `$XDG_CONFIG_HOME/quarry/config.toml` when `XDG_CONFIG_HOME` is set.

A minimal user configuration can contain only the settings you want to change:

```toml
browser = "firefox"
```

The default search engines and favorites will still be available.

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

Clone the repository and run Quarry directly with Cargo:

```bash
git clone <repository-url>
cd quarry
cargo run -- help
```

Example search:

```bash
cargo run -- y "hollow knight"
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
│   │   ├── help.rs
│   │   └── mod.rs
│   ├── core/
│   │   ├── config.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
└── Cargo.toml
```

## Status

Quarry is currently an early-stage project. Core search and configuration functionality is implemented, while additional management commands and polish are still being developed.

## License

Quarry is free software licensed under the GNU General Public License
as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

See [LICENSE](LICENSE) for the full license text.


