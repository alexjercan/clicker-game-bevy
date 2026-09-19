use std::f32::consts::FRAC_PI_2;

use bevy::{
    asset::RenderAssetUsages,
    camera::ScalingMode,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use bevy_rand::prelude::*;
use clicker_animation::{click_tile as click_tile_animation, spawn_tile as spawn_tile_animation};
use clicker_assets::{FadingMaterial, GameAssets, GameState, BACKGROUND_DARK_COLOR};
use hexx::*;
use rand_core::Rng;

use crate::{interaction::ClickedSelectedEvent, progression::SkillPoints, GameplaySystems};

const WORLD_HALF_RADIUS: u32 = 3;

#[derive(Component, Debug, Clone)]
pub(crate) struct HexTile;

#[derive(Component, Debug, Clone)]
pub(crate) struct HexGhost;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub(crate) struct HexCoord(pub(crate) Hex);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum HexTileKind {
    Empty,
    Tree,
    Stone,
    Wheat,
}

impl HexTileKind {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.next_u32() % 4 {
            0 => Self::Empty,
            1 => Self::Tree,
            2 => Self::Stone,
            _ => Self::Wheat,
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct HexMap(pub(crate) HexLayout);

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HexMap(HexLayout {
            orientation: HexOrientation::Flat,
            scale: Vec2::splat(2.0 / 3.0f32.sqrt()),
            ..default()
        }))
        .add_systems(OnEnter(GameState::Playing), setup_world)
        .add_systems(
            Update,
            (clicked_tile_tween, clicked_ghost_spawn).in_set(GameplaySystems::ClickFeedback),
        );
    }
}

fn setup_world(
    mut commands: Commands,
    hexmap: Res<HexMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FadingMaterial>>,
) {
    commands.spawn((
        Name::new("Camera3D"),
        Camera3d::default(),
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

    spawn_ghost(
        &mut commands,
        Hex::ZERO,
        &hexmap,
        0.5,
        &mut meshes,
        &mut materials,
    );
}

fn clicked_tile_tween(
    mut commands: Commands,
    mut events: MessageReader<ClickedSelectedEvent<HexTile>>,
) {
    for event in events.read() {
        commands
            .entity(event.entity())
            .insert(click_tile_animation());
    }
}

#[derive(bevy::ecs::system::SystemParam)]
struct WorldQueries<'w, 's> {
    ghosts: Query<'w, 's, (Entity, &'static HexCoord), With<HexGhost>>,
    tiles: Query<'w, 's, (Entity, &'static HexCoord), With<HexTile>>,
}

#[derive(bevy::ecs::system::SystemParam)]
struct WorldAssets<'w> {
    game: Res<'w, GameAssets>,
    gltfs: Res<'w, Assets<Gltf>>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<FadingMaterial>>,
}

fn clicked_ghost_spawn(
    mut commands: Commands,
    hexmap: Res<HexMap>,
    mut skill_points: ResMut<SkillPoints>,
    mut events: MessageReader<ClickedSelectedEvent<HexGhost>>,
    queries: WorldQueries,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut assets: WorldAssets,
) {
    for event in events.read() {
        let Ok((entity, hex_coord)) = queries.ghosts.get(event.entity()) else {
            continue;
        };
        if **skill_points == 0 {
            continue;
        }

        commands.entity(entity).despawn();
        spawn_tile(
            &mut commands,
            **hex_coord,
            &hexmap,
            &assets.game,
            &assets.gltfs,
            &mut rng,
        );

        for coord in hex_coord
            .ring(1)
            .filter(|coord| {
                queries
                    .ghosts
                    .iter()
                    .all(|(_, HexCoord(ghost_coord))| ghost_coord != coord)
                    && queries
                        .tiles
                        .iter()
                        .all(|(_, HexCoord(tile_coord))| tile_coord != coord)
            })
            .filter(|coord| is_in_world(*coord))
        {
            spawn_ghost(
                &mut commands,
                coord,
                &hexmap,
                1.0,
                &mut assets.meshes,
                &mut assets.materials,
            );
        }

        **skill_points -= 1;
    }
}

fn spawn_tile(
    commands: &mut Commands,
    coord: Hex,
    hexmap: &HexMap,
    game_assets: &GameAssets,
    gltf_assets: &Assets<Gltf>,
    rng: &mut WyRand,
) {
    let translation = hexmap.hex_to_world_pos(coord).extend(0.0).xzy();

    commands
        .spawn((
            Name::new("HexTile"),
            HexTile,
            Visibility::default(),
            Transform::from_translation(translation),
            HexCoord(coord),
            DespawnOnExit(GameState::Playing),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("HexTileRender"),
                    Visibility::default(),
                    Transform::from_xyz(0.0, -5.0, 0.0)
                        .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
                    spawn_tile_animation(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("HexTileMesh"),
                        Transform::default(),
                        GlobalTransform::default(),
                        WorldAssetRoot(
                            gltf_assets.get(&game_assets.hex_base).unwrap().scenes[0].clone(),
                        ),
                    ));
                    spawn_tile_decoration(
                        parent,
                        HexTileKind::random(rng),
                        game_assets,
                        gltf_assets,
                    );
                });
        });
}

fn spawn_tile_decoration(
    parent: &mut ChildSpawnerCommands,
    kind: HexTileKind,
    game_assets: &GameAssets,
    gltf_assets: &Assets<Gltf>,
) {
    let mut spawn_asset = |name, handle: &Handle<Gltf>| {
        parent.spawn((
            Name::new(name),
            Transform::default(),
            GlobalTransform::default(),
            WorldAssetRoot(gltf_assets.get(handle).unwrap().scenes[0].clone()),
        ));
    };

    match kind {
        HexTileKind::Empty => {}
        HexTileKind::Tree => spawn_asset("HexTreeMesh", &game_assets.hex_tree),
        HexTileKind::Stone => spawn_asset("HexStoneMesh", &game_assets.hex_stone),
        HexTileKind::Wheat => {
            spawn_asset("HexDirtMesh", &game_assets.hex_dirt);
            spawn_asset("HexWheatMesh", &game_assets.hex_wheat);
        }
    }
}

fn spawn_ghost(
    commands: &mut Commands,
    coord: Hex,
    hexmap: &HexMap,
    depth: f32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<FadingMaterial>,
) {
    let mesh = ColumnMeshBuilder::new(&hexmap.0, 1.0)
        .with_subdivisions(10)
        .with_offset(Vec3::NEG_Y * depth)
        .build();
    let translation = hexmap.hex_to_world_pos(coord).extend(0.0).xzy();

    commands.spawn((
        Name::new("HexGhost"),
        HexGhost,
        Visibility::default(),
        Transform::from_translation(translation),
        HexCoord(coord),
        Mesh3d(meshes.add(hexagonal_mesh(mesh))),
        MeshMaterial3d(materials.add(FadingMaterial::new(BACKGROUND_DARK_COLOR))),
        DespawnOnExit(GameState::Playing),
    ));
}

fn is_in_world(coord: Hex) -> bool {
    HexBounds::new(Hex::ZERO, WORLD_HALF_RADIUS).is_in_bounds(coord)
}

fn hexagonal_mesh(mesh_info: MeshInfo) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

#[cfg(test)]
mod tests {
    use rand_core::SeedableRng;

    use super::*;

    #[test]
    fn world_bounds_include_radius_three_and_exclude_radius_four() {
        assert!(is_in_world(Hex::new(3, 0)));
        assert!(is_in_world(Hex::new(-3, 3)));
        assert!(!is_in_world(Hex::new(4, 0)));
        assert!(!is_in_world(Hex::new(-4, 4)));
    }

    #[test]
    fn tile_kinds_follow_the_seeded_wyrand_sequence() {
        let mut rng = WyRand::from_seed([7; 8]);
        let kinds = (0..8)
            .map(|_| HexTileKind::random(&mut rng))
            .collect::<Vec<_>>();

        assert_eq!(
            kinds,
            vec![
                HexTileKind::Wheat,
                HexTileKind::Tree,
                HexTileKind::Wheat,
                HexTileKind::Empty,
                HexTileKind::Tree,
                HexTileKind::Tree,
                HexTileKind::Stone,
                HexTileKind::Empty,
            ]
        );
    }
}
