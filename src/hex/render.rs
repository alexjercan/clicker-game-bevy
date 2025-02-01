use bevy::prelude::*;

use super::{HexTile, HexTileKind};

#[derive(Resource, Clone, Debug, Default)]
pub struct HexRenderAssets {
    pub base: Handle<Gltf>,
    pub tree: Handle<Gltf>,
    pub stone: Handle<Gltf>,
    pub dirt: Handle<Gltf>,
    pub wheat: Handle<Gltf>,
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
    q_hex: Query<(Entity, &HexTileKind), (With<HexTile>, Without<HexTileRender>)>,
    game_assets: Res<HexRenderAssets>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    for (entity, kind) in &q_hex {
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
                            gltf_assets.get(&game_assets.base).unwrap().scenes[0].clone(),
                        ),
                    ))
                    .with_children(|parent| {
                        match kind {
                            HexTileKind::Empty => {
                                parent.spawn((
                                    Name::new("TestingEmptyMesh"),
                                    Transform::from_xyz(0.0, 0.0, 0.0),
                                    GlobalTransform::default(),
                                ));
                            },
                            HexTileKind::Tree => {
                                parent.spawn((
                                    Name::new("TestingTreeMesh"),
                                    Transform::from_xyz(0.0, 0.0, 0.0),
                                    GlobalTransform::default(),
                                    SceneRoot(
                                        gltf_assets.get(&game_assets.tree).unwrap().scenes[0]
                                            .clone(),
                                    ),
                                ));
                            },
                            HexTileKind::Stone => {
                                parent.spawn((
                                    Name::new("TestingStoneMesh"),
                                    Transform::from_xyz(0.0, 0.0, 0.0),
                                    GlobalTransform::default(),
                                    SceneRoot(
                                        gltf_assets.get(&game_assets.stone).unwrap().scenes[0]
                                            .clone(),
                                    ),
                                ));
                            },
                            HexTileKind::Wheat => {
                                parent.spawn((
                                    Name::new("TestingDirtMesh"),
                                    Transform::from_xyz(0.0, 0.0, 0.0),
                                    GlobalTransform::default(),
                                    SceneRoot(
                                        gltf_assets.get(&game_assets.dirt).unwrap().scenes[0]
                                            .clone(),
                                    ),
                                ));
                                parent.spawn((
                                    Name::new("TestingWheatMesh"),
                                    Transform::from_xyz(0.0, 0.0, 0.0),
                                    GlobalTransform::default(),
                                    SceneRoot(
                                        gltf_assets.get(&game_assets.wheat).unwrap().scenes[0]
                                            .clone(),
                                    ),
                                ));
                            },
                        }
                    });
            });
    }
}
