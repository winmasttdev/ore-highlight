// /ores toggles the boxes; /ores walls toggles depth; /ores status reports.
fn ore_highlight_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::official_ore_highlight as m;
    let mut parts = command.split_whitespace();
    let name = parts.next().unwrap_or("");
    if !game.native_on(m::ID) || name != "ores" {
        return;
    }
    *ran = true;
    match parts.next() {
        None => {
            let shown = !m::shown(|| game.native_setting(m::ID, "shown"));
            m::set_shown(shown);
            game.native_set_setting(m::ID, "shown", if shown { "true" } else { "false" });
            game.native_chat(if shown {
                "Ore highlight ON"
            } else {
                "Ore highlight OFF"
            });
        }
        Some(value @ ("on" | "off")) => {
            let shown = value == "on";
            m::set_shown(shown);
            game.native_set_setting(m::ID, "shown", if shown { "true" } else { "false" });
            game.native_chat(if shown {
                "Ore highlight ON"
            } else {
                "Ore highlight OFF"
            });
        }
        Some("walls") => {
            let walls = !m::walls(|| game.native_setting(m::ID, "walls"));
            m::set_walls(walls);
            game.native_set_setting(m::ID, "walls", if walls { "true" } else { "false" });
            game.native_chat(if walls {
                "Ore highlight: through walls"
            } else {
                "Ore highlight: depth tested"
            });
        }
        Some("status") => {
            let shown = m::shown(|| game.native_setting(m::ID, "shown"));
            let walls = m::walls(|| game.native_setting(m::ID, "walls"));
            game.native_chat(&format!(
                "Ore highlight {} / through walls {} (/ores, /ores walls)",
                m::on_off(shown),
                m::on_off(walls)
            ));
        }
        Some(_) => game.native_chat("Usage: /ores [on|off|walls|status]"),
    }
}
