// The key toggles the boxes while playing.
fn ore_highlight_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::official_ore_highlight as m;
    if !pressed
        || !game.native_window()
        || !game.native_on(m::ID)
        || !game.native_playing()
        || key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY)
    {
        return;
    }
    let shown = !m::shown(|| game.native_setting(m::ID, "shown"));
    m::set_shown(shown);
    game.native_set_setting(m::ID, "shown", if shown { "true" } else { "false" });
    game.native_chat(if shown {
        "Ore highlight ON"
    } else {
        "Ore highlight OFF"
    });
}
