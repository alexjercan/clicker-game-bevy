use bevy::{camera::ScalingMode, prelude::*};
use clicker_camera::ShakeCamera;
use clicker_state::GameState;

pub(crate) struct MainScenePlugin;

impl Plugin for MainScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_main_scene);
    }
}

fn setup_main_scene(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera3D"),
        Camera3d::default(),
        ShakeCamera::default(),
        Transform::from_xyz(0.0, 15.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 12.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        DespawnOnExit(GameState::Playing),
    ));

    commands.spawn((
        Name::new("DirectionalLight"),
        DirectionalLight::default(),
        Transform::from_xyz(10.0, 15.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(GameState::Playing),
    ));
}
