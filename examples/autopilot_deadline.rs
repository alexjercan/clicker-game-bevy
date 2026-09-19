use bevy::prelude::*;
use clicker_autopilot::prelude::*;
use clicker_probe::ClickerProbePlugin;
use game::{GameState, HexGhost};

fn main() -> AppExit {
    let mut app = game::app();
    app.add_plugins((
        AutopilotPlugin::new()
            .step("the starting ghost appears before the deadline")
            .until(
                and(state_is(GameState::Playing), entity_count::<HexGhost>(1)),
                10.0,
            )
            .add(),
        ClickerProbePlugin,
    ));
    app.run()
}
