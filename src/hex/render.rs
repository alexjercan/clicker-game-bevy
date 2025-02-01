use bevy::prelude::*;

use super::HexTile;

#[derive(Resource, Clone, Debug, Default)]
pub struct HexRenderAssets {
    pub hex_grass: Handle<Gltf>,
    pub trees_a_large: Handle<Gltf>,
}

#[derive(Component, Debug, Default)]
struct HexTileRender;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexRenderPluginSet;

#[derive(Debug)]
pub struct HexRenderPlugin;

impl Plugin for HexRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HexRenderAssets>();

        app.add_systems(Update, add_hex_render.in_set(HexRenderPluginSet));
    }
}

fn add_hex_render(
    mut commands: Commands,
    q_hex: Query<Entity, (With<HexTile>, Without<HexTileRender>)>,
    game_assets: Res<HexRenderAssets>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    for entity in &q_hex {
        commands
            .entity(entity)
            .insert(HexTileRender)
            .with_children(|parent| {
                parent
                    .spawn((
                        Name::new("TestingTileMesh"),
                        Transform::from_xyz(0.0, 0.0, 0.0),
                        GlobalTransform::default(),
                        SceneRoot(
                            gltf_assets.get(&game_assets.hex_grass).unwrap().scenes[0].clone(),
                        ),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("TestingTreeMesh"),
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            GlobalTransform::default(),
                            SceneRoot(
                                gltf_assets.get(&game_assets.trees_a_large).unwrap().scenes[0]
                                    .clone(),
                            ),
                        ));
                    });
            });
    }
}
