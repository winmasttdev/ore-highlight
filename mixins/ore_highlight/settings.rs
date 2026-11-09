// The mod's settings page: highlight on/off, and depth or through walls.
fn ore_highlight_page(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::official_ore_highlight as m;
    if mod_id != m::ID {
        return;
    }
    let shown = m::shown(|| game.native_setting(m::ID, "shown"));
    let walls = m::walls(|| game.native_setting(m::ID, "walls"));
    buttons.push(format!("Highlight: {}", m::on_off(shown)));
    buttons.push(format!("Through walls: {}", m::on_off(walls)));
}
