// A click on the mod's settings page.
fn ore_highlight_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    use crate::game::official_ore_highlight as m;
    if mod_id != m::ID {
        return;
    }
    match button {
        0 => {
            let shown = !m::shown(|| game.native_setting(m::ID, "shown"));
            m::set_shown(shown);
            game.native_set_setting(m::ID, "shown", if shown { "true" } else { "false" });
        }
        1 => {
            let walls = !m::walls(|| game.native_setting(m::ID, "walls"));
            m::set_walls(walls);
            game.native_set_setting(m::ID, "walls", if walls { "true" } else { "false" });
        }
        _ => {}
    }
}
