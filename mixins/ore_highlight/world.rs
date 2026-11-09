// Draws the miner's scan as boxes in the world, once a frame.
fn ore_highlight_world(
    game: &crate::game::Game<'_>,
    primitives: &mut horizon_client::world_primitives::WorldPrimitives,
) {
    use crate::game::official_ore_highlight as m;
    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    if !m::shown(|| game.native_setting(m::ID, "shown")) {
        return;
    }
    let Some(player) = game.native_player() else {
        return;
    };
    let path = game.native_data_dir(m::ID).join("highlight.txt");
    let walls = m::walls(|| game.native_setting(m::ID, "walls"));
    m::with_cached(&path, |scan| {
        let Some(scan) = scan else {
            return;
        };
        // A scan from another dimension would put boxes in the wrong place
        // after a portal. The miner rewrites the file on its next pass.
        if !scan.dimension.is_empty() && scan.dimension != player.dimension {
            return;
        }
        for ore in &scan.ores {
            let min = [
                ore.pos[0] as f64,
                ore.pos[1] as f64,
                ore.pos[2] as f64,
            ];
            let max = [min[0] + 1.0, min[1] + 1.0, min[2] + 1.0];
            let primitive = primitives.box_outline(min, max, m::ore_color(&ore.name), 1.5);
            if walls {
                primitive.through_walls();
            }
        }
        if let Some(target) = scan.target {
            let min = [target[0] as f64, target[1] as f64, target[2] as f64];
            let max = [min[0] + 1.0, min[1] + 1.0, min[2] + 1.0];
            let fill = primitives.filled_box(min, max, 0x30_FFFF_FF);
            if walls {
                fill.through_walls();
            }
            let outline = primitives.box_outline(min, max, 0xFFFF_FFFF, 2.5);
            if walls {
                outline.through_walls();
            }
            let label = primitives.text(
                [min[0] + 0.5, min[1] + 1.5, min[2] + 0.5],
                "TARGET",
                0xFFFF_FFFF,
                0.7,
            );
            label.scale_with_distance().background();
            if walls {
                label.through_walls();
            }
        }
    });
}
