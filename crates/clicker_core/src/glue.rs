use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use clicker_animation::{
    tile_click_animation, tile_deselect_animation, tile_select_animation, tile_spawn_animation,
    AnimationsEnabled, SpawnAnimationFinished,
};
use clicker_assets::{FadingMaterial, GameAssets, BACKGROUND_DARK_COLOR};
use clicker_camera::{CameraFeedbackSystems, CameraImpulse, CameraShakeSettings};
use clicker_state::GameState;
use clicker_tile::{
    HexCoord, HexGhost, HexMap, HexTile, InitializeTileWorld, TileClicked, TileDeselected,
    TileKind, TilePlaced, TilePointer, TileSelected, TileSettled, TileSystems,
};
use hexx::{ColumnMeshBuilder, MeshInfo};

type TileEntity = Or<(With<HexGhost>, With<HexTile>)>;

#[derive(Component)]
struct SpawnedTile(clicker_tile::TileCoord);

pub(crate) struct RenderedTileGluePlugin;

impl Plugin for RenderedTileGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_tile_world)
            .add_systems(OnExit(GameState::Playing), cleanup_tile_world)
            .add_systems(
                Update,
                inject_pointer
                    .before(TileSystems::Selection)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (render_added_ghosts, render_added_tiles)
                    .after(TileSystems::Mutation)
                    .run_if(in_state(GameState::Playing)),
            )
            .configure_sets(
                Update,
                CameraFeedbackSystems::Impulse.after(TileSystems::Feedback),
            )
            .add_systems(
                Update,
                request_camera_impulses
                    .in_set(TileSystems::Feedback)
                    .run_if(in_state(GameState::Playing)),
            );
        if app.world().contains_resource::<AnimationsEnabled>() {
            app.add_systems(
                Update,
                (
                    animate_selected_tiles,
                    animate_deselected_tiles,
                    animate_clicked_tiles,
                    report_settled_tiles,
                )
                    .in_set(TileSystems::Feedback)
                    .run_if(in_state(GameState::Playing)),
            );
        }
    }
}

pub(crate) struct HeadlessTileGluePlugin;

impl Plugin for HeadlessTileGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_tile_world)
            .add_systems(OnExit(GameState::Playing), cleanup_tile_world)
            .add_systems(Update, settle_placed_tiles.after(TileSystems::Mutation));
    }
}

fn initialize_tile_world(mut events: MessageWriter<InitializeTileWorld>) {
    events.write(InitializeTileWorld);
}

fn cleanup_tile_world(mut commands: Commands, entities: Query<Entity, TileEntity>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn inject_pointer(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut pointer: ResMut<TilePointer>,
) {
    pointer.activate = buttons.just_pressed(MouseButton::Left);
    pointer.world_position = pointer_world_position(&windows, &camera);
}

fn pointer_world_position(
    windows: &Query<&Window>,
    camera: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera.single().ok()?;
    let window = windows.single().ok()?;
    let cursor_position = window.cursor_position()?;
    let ray = camera
        .viewport_to_world(camera_transform, cursor_position)
        .ok()?;
    let distance = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))?;
    Some(ray.get_point(distance).xz())
}

fn animate_selected_tiles(mut commands: Commands, mut events: MessageReader<TileSelected>) {
    for event in events.read() {
        commands
            .entity(event.entity)
            .insert(tile_select_animation());
    }
}

fn animate_deselected_tiles(mut commands: Commands, mut events: MessageReader<TileDeselected>) {
    for event in events.read() {
        commands
            .entity(event.entity)
            .insert(tile_deselect_animation());
    }
}

fn animate_clicked_tiles(mut commands: Commands, mut events: MessageReader<TileClicked>) {
    for event in events.read() {
        commands.entity(event.entity).insert(tile_click_animation());
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

fn report_settled_tiles(
    mut completed: MessageReader<SpawnAnimationFinished>,
    tiles: Query<&SpawnedTile>,
    mut settled: MessageWriter<TileSettled>,
) {
    for event in completed.read() {
        let Ok(coord) = tiles.get(event.entity) else {
            continue;
        };
        settled.write(TileSettled { coord: coord.0 });
    }
}

fn settle_placed_tiles(
    mut placed: MessageReader<TilePlaced>,
    mut settled: MessageWriter<TileSettled>,
) {
    for event in placed.read() {
        settled.write(TileSettled { coord: event.coord });
    }
}

fn render_added_ghosts(
    mut commands: Commands,
    hexmap: Res<HexMap>,
    ghosts: Query<(Entity, &HexCoord), Added<HexGhost>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FadingMaterial>>,
) {
    for (entity, coord) in &ghosts {
        let depth = if coord.tile_coord() == clicker_tile::TileCoord::ZERO {
            0.5
        } else {
            1.0
        };
        let mesh = ColumnMeshBuilder::new(hexmap.layout(), 1.0)
            .with_subdivisions(10)
            .with_offset(Vec3::NEG_Y * depth)
            .build();
        let translation = hexmap.world_position(coord.tile_coord()).extend(0.0).xzy();
        commands.entity(entity).insert((
            Visibility::default(),
            Transform::from_translation(translation),
            Mesh3d(meshes.add(hexagonal_mesh(mesh))),
            MeshMaterial3d(materials.add(FadingMaterial::new(BACKGROUND_DARK_COLOR))),
        ));
    }
}

fn render_added_tiles(
    mut commands: Commands,
    hexmap: Res<HexMap>,
    tiles: Query<(Entity, &HexCoord, &TileKind), Added<HexTile>>,
    game_assets: Res<GameAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    animations: Option<Res<AnimationsEnabled>>,
    mut settled: MessageWriter<TileSettled>,
) {
    for (entity, coord, kind) in &tiles {
        let translation = hexmap.world_position(coord.tile_coord()).extend(0.0).xzy();
        let mut tile = commands.entity(entity);
        tile.insert((
            Visibility::default(),
            Transform::from_translation(translation),
        ));
        tile.with_children(|parent| {
            let mut render = parent.spawn((
                Name::new("HexTileRender"),
                Visibility::default(),
                Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            ));
            if animations.is_some() {
                render.insert((SpawnedTile(coord.tile_coord()), tile_spawn_animation()));
            }
            render.with_children(|parent| {
                parent.spawn((
                    Name::new("HexTileMesh"),
                    Transform::default(),
                    GlobalTransform::default(),
                    WorldAssetRoot(
                        gltf_assets.get(&game_assets.hex_base).unwrap().scenes[0].clone(),
                    ),
                ));
                spawn_tile_decoration(parent, *kind, &game_assets, &gltf_assets);
            });
        });
        if animations.is_none() {
            settled.write(TileSettled {
                coord: coord.tile_coord(),
            });
        }
    }
}

fn spawn_tile_decoration(
    parent: &mut ChildSpawnerCommands,
    kind: TileKind,
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
        TileKind::Empty => {}
        TileKind::Tree => spawn_asset("HexTreeMesh", &game_assets.hex_tree),
        TileKind::Stone => spawn_asset("HexStoneMesh", &game_assets.hex_stone),
        TileKind::Wheat => {
            spawn_asset("HexDirtMesh", &game_assets.hex_dirt);
            spawn_asset("HexWheatMesh", &game_assets.hex_wheat);
        }
    }
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
