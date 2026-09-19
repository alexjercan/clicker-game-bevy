use std::f32::consts::PI;

use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(Debug, Resource, Default, Clone, Deref, DerefMut)]
struct ShowAxes(pub bool);

#[derive(Component)]
struct PerfUi;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(EntityCountDiagnosticsPlugin::default())
            .add_plugins(SystemInformationDiagnosticsPlugin)
            .insert_resource(ShowAxes(true))
            .add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Startup, setup)
            .add_systems(Update, toggle)
            .add_systems(
                Update,
                (
                    draw_axes,
                    draw_hexmap,
                    draw_cursor,
                    add_ui_border,
                    update_perf_ui,
                ),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        PerfUi,
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
        ZIndex(100),
    ));
}

fn toggle(
    kbd: Res<ButtonInput<KeyCode>>,
    mut show_axes: ResMut<ShowAxes>,
    mut visibility: Single<&mut Visibility, With<PerfUi>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        show_axes.0 = !show_axes.0;
        **visibility = if show_axes.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_perf_ui(diagnostics: Res<DiagnosticsStore>, mut text: Single<&mut Text, With<PerfUi>>) {
    let value = |path| {
        diagnostics
            .get(path)
            .and_then(Diagnostic::smoothed)
            .unwrap_or_default()
    };

    text.0 = format!(
        "FPS: {:.0}\nFrame: {:.2} ms\nEntities: {:.0}\nCPU: {:.1}%\nMemory: {:.1} MiB",
        value(&FrameTimeDiagnosticsPlugin::FPS),
        value(&FrameTimeDiagnosticsPlugin::FRAME_TIME),
        value(&EntityCountDiagnosticsPlugin::ENTITY_COUNT),
        value(&SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE),
        value(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE),
    );
}

fn draw_axes(mut gizmos: Gizmos, query: Query<&Transform>, show_axes: Res<ShowAxes>) {
    if !show_axes.0 {
        return;
    }

    for &transform in &query {
        gizmos.axes(transform, 3.0);
    }
}

fn draw_hexmap(mut gizmos: Gizmos, show_axes: Res<ShowAxes>) {
    if !show_axes.0 {
        return;
    }

    let size = 2.0 / 3.0f32.sqrt();
    for r in -10..=10 {
        for q in -10..=10 {
            let x = size * 3.0 / 2.0 * q as f32;
            let z = size * 3.0f32.sqrt() * (q as f32 / 2.0 + r as f32);

            let mut direction = Vec3::new(-size, 0.0, 0.0);
            let rotation = Quat::from_rotation_y(PI / 3.0);
            for _ in 0..6 {
                let prev = Vec3::new(x, 0.0, z) + direction;
                direction = rotation.mul_vec3(direction);
                let next = Vec3::new(x, 0.0, z) + direction;
                gizmos.line(prev, next, Color::WHITE);
            }
        }
    }
}

fn draw_cursor(
    q_camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {
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

fn add_ui_border(mut commands: Commands, q_node: Query<Entity, (With<Node>, Without<Outline>)>) {
    for entity in q_node.iter() {
        commands
            .entity(entity)
            .insert(Outline::new(Val::Px(1.0), Val::Px(0.0), Color::WHITE));
    }
}
