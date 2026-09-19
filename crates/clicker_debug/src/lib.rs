use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(Resource, Default)]
struct DebugUi {
    visible: bool,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugUi>()
            .add_plugins(EguiPlugin::default())
            .add_plugins(FpsOverlayPlugin {
                config: fps_overlay_config(),
            })
            .add_plugins(WorldInspectorPlugin::new().run_if(debug_visible))
            .add_systems(Update, toggle_debug_ui)
            .add_systems(Update, draw_cursor.run_if(debug_visible));
    }
}

fn fps_overlay_config() -> FpsOverlayConfig {
    FpsOverlayConfig {
        enabled: false,
        frame_time_graph_config: FrameTimeGraphConfig {
            enabled: false,
            ..default()
        },
        ..default()
    }
}

fn debug_visible(state: Res<DebugUi>) -> bool {
    state.visible
}

fn toggle_debug_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugUi>,
    mut overlay: ResMut<FpsOverlayConfig>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        state.visible = !state.visible;
        overlay.enabled = state.visible;
    }
}

fn draw_cursor(
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };

    gizmos.cross(ray.get_point(distance) + Vec3::Y * 0.01, 0.5, Color::WHITE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f11_toggles_debug_ui_and_fps_overlay_together() {
        assert!(!fps_overlay_config().frame_time_graph_config.enabled);
        let mut app = App::new();
        app.init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugUi>()
            .init_resource::<FpsOverlayConfig>()
            .add_systems(Update, toggle_debug_ui);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::F11);
        app.update();
        assert!(app.world().resource::<DebugUi>().visible);
        assert!(app.world().resource::<FpsOverlayConfig>().enabled);
    }
}
