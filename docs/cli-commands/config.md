# openisl config

View and update openISL settings.

## Synopsis

```bash
openisl config [OPTIONS]
```

## Description

Reads and writes the user configuration file at `~/.config/openisl/config.toml`. With no options, it prints the current configuration and then saves it.

## Options

| Option | Description |
|--------|-------------|
| `--show` | Print the current configuration |
| `--reset` | Reset the configuration file to defaults |
| `--theme <dark\|light>` | Set the TUI theme |
| `--max-commits <N>` | Set the maximum number of commits loaded by the TUI |
| `-h, --help` | Show help |

## Examples

Show the current configuration:

```bash
openisl config --show
```

Set the theme:

```bash
openisl config --theme dark
```

Increase the number of commits loaded by the TUI:

```bash
openisl config --max-commits 500
```

Reset everything to defaults:

```bash
openisl config --reset
```

## Configuration File

`openisl config` writes `~/.config/openisl/config.toml`. All fields:

```toml
[general]
max_commits = 100
date_format = "%Y-%m-%d %H:%M:%S UTC"
verbose = false

[tui]
theme = "dark"          # dark | light
page_size = 20
show_help_on_start = false

[git]
auto_fetch = false
fetch_remotes = false
```

Settings can also be overridden per invocation with environment variables using the `OPENISL_` prefix, e.g. `OPENISL_TUI_THEME=light openisl tui`.

## See Also

- [openisl tui](tui.md) - The TUI whose defaults are configured here