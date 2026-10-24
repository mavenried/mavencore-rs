<h1 align="center">mavencore-rs</h1>
<p align="center">The Rust backend for <a href="https://github.com/mavenried/mavencore-shell"><code>mavencore-shell</code></a></p>
<hr>

`mavencore` provides the system data and compositor events that mavencore-shell's Quickshell components consume. The shell invokes it as a subprocess for status-bar readings, battery details, installed applications, dictionary lookups, and live workspace/window state.

## Build

Build the release binary with Rust and Cargo:

```sh
cargo build --release
```

Install `target/release/mavencore` somewhere on `PATH` in the environment that launches Quickshell. The shell expects to invoke it by the name `mavencore`.

## Shell integration

| Shell feature                     | Backend command                                                                     | Output                                                      |
| --------------------------------- | ----------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| Bar and dashboard system stats    | `mavencore cpu`, `memory`, `disk <path>`, `uptime`                                  | Numeric readings; disk takes a mount path.                  |
| Battery status and power display  | `mavencore battery <sysfs-path>`, `power <sysfs-path>`, `battery-icon <sysfs-path>` | Charge percentage, watts, and an icon glyph.                |
| Bar network indicator             | `mavencore online`                                                                  | Reachability icon after a TCP check of `1.1.1.1:80`.        |
| App launcher                      | `mavencore apps-list`                                                               | One JSON application record per line.                       |
| Launcher dictionary mode          | `mavencore dict <word>`                                                             | HTML-formatted English Wiktionary definitions.              |
| Active workspace and window title | `mavencore listen`                                                                  | Continuous JSON state snapshots from the active compositor. |

The shell's QML calls these commands directly and parses their stdout, so output is intended for shell integration rather than polished interactive terminal use. Most numeric readings omit the `%` sign. The `listen` command is a long-running event stream; the other commands print a result and exit.

## Runtime requirements

- Linux, for `/proc`, power-supply sysfs, and filesystem statistics.
- A supported Wayland compositor and its session environment for `listen`: Niri or Hyprland. `XDG_CURRENT_DESKTOP` selects the handler.
- A Nerd Font or compatible icon font to render the glyphs from `online` and `battery-icon`.
- Network access for `dict`; it queries the English Wiktionary REST API.

`apps-list` searches application directories under `XDG_DATA_DIRS` and `~/.local/share/applications`, honoring `Hidden`, `NoDisplay`, `OnlyShowIn`, and `NotShowIn` desktop-entry keys. Battery commands take a power-supply directory such as `/sys/class/power_supply/BAT1`.

Run `mavencore --help` to see the command-line interface.
