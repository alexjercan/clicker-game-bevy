use std::sync::Arc;

use bevy::prelude::*;
use clicker_autopilot::prelude::*;
use clicker_probe::prelude::*;
use game::{
    ClickTile, GameState, HexGhost, HexTile, SkillPoints, TileCoord, TileSettled, XpMax, XpValue,
};

const STEP_DEADLINE: f32 = 10.0;

#[derive(Resource, Default)]
struct SettledTiles(u32);

fn main() -> AppExit {
    let mut app = game::app();
    app.init_resource::<SettledTiles>()
        .add_systems(Update, track_settled_tiles)
        .add_plugins((script(), ClickerProbePlugin));
    app.run()
}

fn track_settled_tiles(mut events: MessageReader<TileSettled>, mut settled: ResMut<SettledTiles>) {
    settled.0 += events.read().count() as u32;
}

fn script() -> AutopilotPlugin {
    let mut script = AutopilotPlugin::new()
        .step("the playable tile world is ready")
        .until(
            and(state_is(GameState::Playing), entity_count::<HexGhost>(1)),
            STEP_DEADLINE,
        )
        .add()
        .step("the first tile is placed")
        .act(click(TileCoord::ZERO))
        .until(entity_count::<HexTile>(1), STEP_DEADLINE)
        .add();

    for click_index in 1..=10 {
        let name = if click_index == 10 {
            "ten tile clicks roll XP over".to_string()
        } else {
            format!("tile click {click_index} awards XP")
        };
        script = script
            .step(name)
            .act(click(TileCoord::ZERO))
            .until(
                Arc::new(move |world| {
                    let xp = world.resource::<XpValue>().0;
                    let max = world.resource::<XpMax>().0;
                    let points = world.resource::<SkillPoints>().0;
                    if click_index < 10 {
                        xp == click_index && max == 10 && points == 0
                    } else {
                        xp == 0 && max == 20 && points == 1
                    }
                }),
                STEP_DEADLINE,
            )
            .add();
    }

    script = script
        .step("the earned skill point places a second tile")
        .act(click(TileCoord::new(-1, 0)))
        .until(
            and(
                entity_count::<HexTile>(2),
                resource_where::<SettledTiles>(|settled| settled.0 == 2),
            ),
            STEP_DEADLINE,
        )
        .add()
        .step("the completed world satisfies the gameplay verdict")
        .act(|world| {
            marker(
                world,
                "gameplay verdict",
                serde_json::json!({"tiles": 2, "xp_max": 20}),
            );
            probe_snapshot(world, "gameplay verdict");
        })
        .expect(Arc::new(|world| {
            let xp = world.resource::<XpValue>().0;
            let max = world.resource::<XpMax>().0;
            let points = world.resource::<SkillPoints>().0;
            xp == 0 && max == 20 && points == 0
        }))
        .add();

    script
}

fn click(coord: TileCoord) -> impl Fn(&mut World) + Send + Sync + 'static {
    move |world| {
        world.write_message(ClickTile(coord));
    }
}
