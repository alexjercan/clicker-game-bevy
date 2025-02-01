use bevy::prelude::*;

use crate::core::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LightPluginSet;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_light.in_set(LightPluginSet).run_if(run_once),
        );
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
