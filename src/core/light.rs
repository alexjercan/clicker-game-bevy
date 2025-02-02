//! Light plugin for the game.

use bevy::prelude::*;

use crate::core::*;

pub(super) struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Playing), setup_light);
    }
}

fn setup_light(mut commands: Commands) {
    commands.spawn((
        Name::new("DirectionalLight"),
        DirectionalLight::default(),
        Transform::from_xyz(-1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        StateScoped(GameStates::Playing),
    ));
}
