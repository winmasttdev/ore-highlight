// Horizon Ore Highlight: boxes around every ore the miner has scanned, drawn
// through the world so you can watch the bot work. The miner publishes its
// current scan to `horizon-mods/.horizon-mod-data/ore_highlight/highlight.txt`
// and this mod reads it, so it works on a public server: it never reads the
// world itself and never sends anything.
//
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game.
use std::cell::Cell;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "ore_highlight";
pub const NAME: &str = "Ore Highlight";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Boxes every ore the miner scanned, through walls. /ores toggles it; /ores walls toggles depth.";
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.o";

/// One ore block from the miner's scan.
#[derive(Clone, Debug, PartialEq)]
pub struct Ore {
    pub pos: [i32; 3],
    pub name: String,
}

/// One published scan.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Highlight {
    pub dimension: String,
    pub player: Option<[f64; 3]>,
    pub target: Option<[i32; 3]>,
    pub ores: Vec<Ore>,
}

/// The colour of an ore, ARGB. Deepslate variants share their stone cousin's
/// colour so the pair reads as one ore.
pub fn ore_color(name: &str) -> u32 {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);
    let name = name.strip_prefix("deepslate_").unwrap_or(name);
    match name {
        "diamond_ore" => 0xFF00_E5FF,
        "emerald_ore" => 0xFF00_FF66,
        "gold_ore" | "nether_gold_ore" | "gilded_blackstone" => 0xFFFF_D700,
        "iron_ore" => 0xFFE0_B080,
        "redstone_ore" => 0xFFFF_3030,
        "lapis_ore" => 0xFF30_60FF,
        "copper_ore" => 0xFFFF_8C42,
        "coal_ore" => 0xFFCC_CCCC,
        "nether_quartz_ore" => 0xFFFF_FFFF,
        "ancient_debris" => 0xFFB0_6030,
        _ => 0xFFFF_00FF,
    }
}

fn int3(it: &mut std::str::SplitWhitespace<'_>) -> Option<[i32; 3]> {
    let x = it.next()?.parse().ok()?;
    let y = it.next()?.parse().ok()?;
    let z = it.next()?.parse().ok()?;
    Some([x, y, z])
}

/// Reads the miner's line format:
///
/// ```text
/// # horizon-ore-highlight 1
/// dimension minecraft:overworld
/// player 12.5 64.0 -3.5
/// target 12 62 -3
/// ore 12 62 -3 minecraft:deepslate_gold_ore
/// ```
pub fn parse(text: &str) -> Highlight {
    let mut out = Highlight::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        match it.next() {
            Some("dimension") => {
                out.dimension = it.next().unwrap_or("").to_owned();
            }
            Some("player") => {
                let x = it.next().and_then(|v| v.parse().ok());
                let y = it.next().and_then(|v| v.parse().ok());
                let z = it.next().and_then(|v| v.parse().ok());
                if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                    out.player = Some([x, y, z]);
                }
            }
            Some("target") => out.target = int3(&mut it),
            Some("ore") => {
                if let Some(pos) = int3(&mut it) {
                    if let Some(name) = it.next() {
                        out.ores.push(Ore {
                            pos,
                            name: name.to_owned(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    out
}

struct Cache {
    path: String,
    checked: Instant,
    modified: Option<SystemTime>,
    value: Option<Highlight>,
}

static CACHE: Mutex<Option<Cache>> = Mutex::new(None);

/// Runs `f` with the last scan, re-reading the file when its modification
/// time changes. Checked at most every 400 ms, so the render thread does no
/// file work on most frames. Never panics on a poisoned lock or a bad file.
pub fn with_cached<R>(path: &Path, f: impl FnOnce(Option<&Highlight>) -> R) -> R {
    let key = path.to_string_lossy().into_owned();
    let now = Instant::now();
    let mut guard = match CACHE.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    let cache = match guard.as_mut() {
        Some(cache) if cache.path == key => cache,
        _ => {
            *guard = Some(Cache {
                path: key,
                checked: now,
                modified: None,
                value: None,
            });
            guard.as_mut().expect("just set")
        }
    };
    if now.duration_since(cache.checked) >= Duration::from_millis(400) {
        cache.checked = now;
        let modified = std::fs::metadata(path).ok().and_then(|m| m.modified().ok());
        if modified != cache.modified {
            cache.modified = modified;
            cache.value = std::fs::read_to_string(path).ok().map(|text| parse(&text));
        }
    }
    f(cache.value.as_ref())
}

thread_local! {
    /// Whether the boxes show, read once from its setting file, then kept.
    static SHOWN: Cell<Option<bool>> = const { Cell::new(None) };
    /// Whether the boxes show through walls.
    static WALLS: Cell<Option<bool>> = const { Cell::new(None) };
}

pub fn read_flag(stored: Option<String>, default: bool) -> bool {
    match stored {
        Some(value) => match value.trim() {
            "false" => false,
            "true" => true,
            _ => default,
        },
        None => default,
    }
}

pub fn shown(get: impl FnOnce() -> Option<String>) -> bool {
    SHOWN.with(|s| {
        let found = s.get().unwrap_or_else(|| read_flag(get(), true));
        s.set(Some(found));
        found
    })
}

pub fn set_shown(value: bool) {
    SHOWN.with(|s| s.set(Some(value)));
}

pub fn walls(get: impl FnOnce() -> Option<String>) -> bool {
    WALLS.with(|s| {
        let found = s.get().unwrap_or_else(|| read_flag(get(), true));
        s.set(Some(found));
        found
    })
}

pub fn set_walls(value: bool) {
    WALLS.with(|s| s.set(Some(value)));
}

pub fn on_off(value: bool) -> &'static str {
    if value { "ON" } else { "OFF" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_miner_file() {
        let text = "# horizon-ore-highlight 1\n\
                    dimension minecraft:overworld\n\
                    player 12.5 64.0 -3.5\n\
                    target 12 62 -3\n\
                    ore 12 62 -3 minecraft:deepslate_gold_ore\n\
                    ore -4 17 8 minecraft:diamond_ore\n";
        let h = parse(text);
        assert_eq!(h.dimension, "minecraft:overworld");
        assert_eq!(h.player, Some([12.5, 64.0, -3.5]));
        assert_eq!(h.target, Some([12, 62, -3]));
        assert_eq!(
            h.ores,
            vec![
                Ore {
                    pos: [12, 62, -3],
                    name: "minecraft:deepslate_gold_ore".into()
                },
                Ore {
                    pos: [-4, 17, 8],
                    name: "minecraft:diamond_ore".into()
                },
            ]
        );
    }

    #[test]
    fn skips_broken_lines() {
        let h = parse("ore 1 2\nore a b c minecraft:coal_ore\nore 1 2 3\n");
        assert!(h.ores.is_empty());
    }

    #[test]
    fn deepslate_keeps_the_colour() {
        for ore in [
            "diamond_ore",
            "emerald_ore",
            "gold_ore",
            "iron_ore",
            "redstone_ore",
            "lapis_ore",
            "copper_ore",
            "coal_ore",
        ] {
            assert_eq!(
                ore_color(&format!("minecraft:{ore}")),
                ore_color(&format!("minecraft:deepslate_{ore}"))
            );
        }
        assert_ne!(ore_color("minecraft:diamond_ore"), ore_color("minecraft:gold_ore"));
    }

    #[test]
    fn flags_default_to_on() {
        assert!(read_flag(None, true));
        assert!(!read_flag(None, false));
        assert!(read_flag(Some("true\n".into()), false));
        assert!(!read_flag(Some("false\n".into()), true));
        assert!(read_flag(Some("junk".into()), true));
        assert_eq!(on_off(true), "ON");
        assert_eq!(on_off(false), "OFF");
    }
}
