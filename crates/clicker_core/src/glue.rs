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
use clicker_audio::{AudioSystems, SfxCue};
use clicker_camera::{CameraFeedbackSystems, CameraImpulse, CameraShakeSettings};
use clicker_gameplay::LevelUp;
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
            .configure_sets(Update, AudioSystems::Playback.after(TileSystems::Feedback))
            .add_systems(
                Update,
                (
                    request_camera_impulses,
                    request_click_sfx,
                    request_selection_sfx,
                    request_placement_sfx,
                    request_level_up_sfx,
                )
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

fn request_click_sfx(
    mut clicks: MessageReader<TileClicked>,
    kinds: Query<&TileKind>,
    mut cues: MessageWriter<SfxCue>,
) {
    for click in clicks.read() {
        if let Ok(kind) = kinds.get(click.entity) {
            cues.write(SfxCue::TileClick(*kind));
        }
    }
}

fn request_selection_sfx(
    mut selected: MessageReader<TileSelected>,
    mut deselected: MessageReader<TileDeselected>,
    mut cues: MessageWriter<SfxCue>,
) {
    let selected_changed = selected.read().count() > 0;
    let deselected_changed = deselected.read().count() > 0;
    if selected_changed || deselected_changed {
        cues.write(SfxCue::Selection);
    }
}

fn request_placement_sfx(mut placed: MessageReader<TilePlaced>, mut cues: MessageWriter<SfxCue>) {
    for _ in placed.read() {
        cues.write(SfxCue::Placement);
    }
}

fn request_level_up_sfx(mut level_ups: MessageReader<LevelUp>, mut cues: MessageWriter<SfxCue>) {
    for _ in level_ups.read() {
        cues.write(SfxCue::LevelUp);
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

#[cfg(test)]
mod tests {
    use super::*;
    use clicker_tile::TileCoord;

    #[derive(Resource, Default)]
    struct CollectedCues(Vec<SfxCue>);

    fn collect_cues(mut cues: MessageReader<SfxCue>, mut collected: ResMut<CollectedCues>) {
        collected.0.extend(cues.read());
    }

    #[test]
    fn tile_click_requests_the_cue_for_its_kind() {
        let mut app = App::new();
        app.add_message::<TileClicked>()
            .add_message::<SfxCue>()
            .init_resource::<CollectedCues>()
            .add_systems(Update, (request_click_sfx, collect_cues).chain());
        let entity = app.world_mut().spawn(TileKind::Stone).id();
        app.world_mut().write_message(TileClicked {
            entity,
            coord: TileCoord::ZERO,
        });

        app.update();

        assert_eq!(
            app.world().resource::<CollectedCues>().0,
            vec![SfxCue::TileClick(TileKind::Stone)]
        );
    }

    #[test]
    fn placement_and_level_up_request_distinct_cues() {
        let mut app = App::new();
        app.add_message::<TilePlaced>()
            .add_message::<LevelUp>()
            .add_message::<SfxCue>()
            .init_resource::<CollectedCues>()
            .add_systems(
                Update,
                (request_placement_sfx, request_level_up_sfx, collect_cues).chain(),
            );
        app.world_mut().write_message(TilePlaced {
            coord: TileCoord::ZERO,
            kind: TileKind::Tree,
        });
        app.world_mut().write_message(LevelUp);

        app.update();

        assert_eq!(
            app.world().resource::<CollectedCues>().0,
            vec![SfxCue::Placement, SfxCue::LevelUp]
        );
    }

    #[test]
    fn simultaneous_deselect_and_select_request_one_selection_cue() {
        let mut app = App::new();
        app.add_message::<TileSelected>()
            .add_message::<TileDeselected>()
            .add_message::<SfxCue>()
            .init_resource::<CollectedCues>()
            .add_systems(Update, (request_selection_sfx, collect_cues).chain());
        let old_entity = app.world_mut().spawn_empty().id();
        let new_entity = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(TileDeselected {
            entity: old_entity,
            coord: TileCoord::ZERO,
        });
        app.world_mut().write_message(TileSelected {
            entity: new_entity,
            coord: TileCoord::new(1, 0),
        });

        app.update();

        assert_eq!(
            app.world().resource::<CollectedCues>().0,
            vec![SfxCue::Selection]
        );
    }
}
