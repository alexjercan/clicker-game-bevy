use bevy::prelude::*;
use clicker_animation::{
    tile_click_animation, tile_deselect_animation, tile_select_animation, tile_spawn_animation,
    SpawnAnimationFinished,
};
use clicker_render::{RenderGhost, RenderSystems, RenderTile, TileRendered};
use clicker_state::GameState;
use clicker_tile::{
    HexCoord, HexGhost, HexMap, HexTile, TileClicked, TileDeselected, TileKind, TilePointer,
    TileSelected, TileSettled, TileSystems,
};

#[derive(Component)]
struct SpawnedTile(clicker_tile::TileCoord);

pub(crate) struct RenderGluePlugin;

impl Plugin for RenderGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            inject_pointer
                .before(TileSystems::Selection)
                .run_if(in_state(GameState::Playing)),
        )
        .configure_sets(Update, RenderSystems::Request.after(TileSystems::Mutation))
        .configure_sets(Update, RenderSystems::Render.before(TileSystems::Feedback))
        .add_systems(
            Update,
            (request_added_ghosts, request_added_tiles)
                .in_set(RenderSystems::Request)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            prepare_rendered_tiles
                .after(RenderSystems::Render)
                .before(TileSystems::Feedback)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
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

fn request_added_ghosts(
    hexmap: Res<HexMap>,
    ghosts: Query<(Entity, &HexCoord), Added<HexGhost>>,
    mut requests: MessageWriter<RenderGhost>,
) {
    for (entity, coord) in &ghosts {
        let depth = if coord.tile_coord() == clicker_tile::TileCoord::ZERO {
            0.5
        } else {
            1.0
        };
        requests.write(RenderGhost {
            entity,
            layout: hexmap.layout().clone(),
            translation: hexmap.world_position(coord.tile_coord()).extend(0.0).xzy(),
            depth,
        });
    }
}

fn request_added_tiles(
    hexmap: Res<HexMap>,
    tiles: Query<(Entity, &HexCoord, &TileKind), Added<HexTile>>,
    mut requests: MessageWriter<RenderTile>,
) {
    for (entity, coord, kind) in &tiles {
        requests.write(RenderTile {
            entity,
            translation: hexmap.world_position(coord.tile_coord()).extend(0.0).xzy(),
            kind: *kind,
        });
    }
}

fn prepare_rendered_tiles(
    mut commands: Commands,
    mut rendered: MessageReader<TileRendered>,
    tiles: Query<&HexCoord>,
) {
    for event in rendered.read() {
        let Ok(coord) = tiles.get(event.tile_entity) else {
            continue;
        };
        commands
            .entity(event.render_entity)
            .insert((SpawnedTile(coord.tile_coord()), tile_spawn_animation()));
    }
}
