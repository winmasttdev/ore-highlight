# Ore Highlight — Horizon native mod

Boxes every ore the miner has scanned, drawn through the world so you can watch the bot work.

On a public server a client mod cannot read arbitrary world blocks, so the miner publishes its own scans to `horizon-mods/.horizon-mod-data/ore_highlight/highlight.txt` and this mod renders them. The miner scans a ~32-block radius every 2 seconds and publishes `dimension / player / target / ore X Y Z name` lines atomically.

## Colours (ARGB, deepslate keeps the stone colour)
diamond cyan, emerald green, gold yellow, iron tan, redstone red, lapis blue, copper orange, coal grey, quartz white, debris brown. Current target: white fill + TARGET tag.

## Controls
`/ores` toggle, `/ores walls` through-walls on/off, `/ores status`, key `O`, Options > Mods page.

## Layout
`native/ore-highlight/` style bundle: `horizon-native.json`, `Cargo.toml`, `compatibility.json`, `mixins/ore_highlight/` (runtime + world/command/key/list/settings/click helpers + `horizon-mixin.toml`). Tests live in `runtime.rs`.

## Build
`python3 native/tools/native.py check|package ore-highlight` (horizon-mods registry) or `horizon-launcher native-build --trust-native-code <folder>`.
