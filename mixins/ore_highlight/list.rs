// The mod's row in Options > Mods, with its toggle key.
fn ore_highlight_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::official_ore_highlight as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[
            m::KEY.into(),
            "Toggle ore highlight".into(),
            m::DEFAULT_KEY.into(),
        ]],
    });
}
