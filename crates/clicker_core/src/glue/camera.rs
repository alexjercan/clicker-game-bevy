use bevy::prelude::*;
use clicker_camera::{CameraFeedbackSystems, CameraImpulse, CameraShakeSettings};
use clicker_gameplay::{TileClicked, TileSystems};
use clicker_state::GameState;

pub(super) struct CameraGluePlugin;

impl Plugin for CameraGluePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            CameraFeedbackSystems::Impulse.after(TileSystems::Feedback),
        )
        .add_systems(
            Update,
            request_camera_impulses
                .in_set(TileSystems::Feedback)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn request_camera_impulses(
    settings: Res<CameraShakeSettings>,
    mut clicks: MessageReader<TileClicked>,
    mut impulses: MessageWriter<CameraImpulse>,
) {
    for _ in clicks.read() {
        impulses.write(CameraImpulse {
            strength: settings.click_impulse,
        });
    }
}
