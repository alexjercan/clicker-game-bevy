use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use clicker_assets::{FadingMaterial, GameAssets, BACKGROUND_DARK_COLOR};
use clicker_tile::TileKind;
use hexx::{ColumnMeshBuilder, HexLayout, MeshInfo};

#[derive(Message, Debug, Clone)]
pub struct RenderGhost {
    pub entity: Entity,
    pub layout: HexLayout,
    pub translation: Vec3,
    pub depth: f32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct RenderTile {
    pub entity: Entity,
    pub translation: Vec3,
    pub kind: TileKind,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct TileRendered {
    pub tile_entity: Entity,
    pub render_entity: Entity,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderSystems {
    Request,
    Render,
}

pub struct ClickerRenderPlugin;

impl Plugin for ClickerRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RenderGhost>()
            .add_message::<RenderTile>()
            .add_message::<TileRendered>()
            .configure_sets(
                Update,
                (RenderSystems::Request, RenderSystems::Render).chain(),
            )
            .add_systems(Update, render_ghosts.in_set(RenderSystems::Render))
            .add_systems(
                Update,
                render_tiles
                    .in_set(RenderSystems::Render)
                    .run_if(resource_exists::<GameAssets>),
            );
    }
}

fn render_ghosts(
    mut commands: Commands,
    mut requests: MessageReader<RenderGhost>,
    targets: Query<()>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FadingMaterial>>,
) {
    for request in requests.read() {
        if !targets.contains(request.entity) {
            continue;
        }

        let mesh = ColumnMeshBuilder::new(&request.layout, 1.0)
            .with_subdivisions(10)
            .with_offset(Vec3::NEG_Y * request.depth)
            .build();
        commands.entity(request.entity).insert((
            Visibility::default(),
            Transform::from_translation(request.translation),
            Mesh3d(meshes.add(hexagonal_mesh(mesh))),
            MeshMaterial3d(materials.add(FadingMaterial::new(BACKGROUND_DARK_COLOR))),
        ));
    }
}

fn render_tiles(
    mut commands: Commands,
    mut requests: MessageReader<RenderTile>,
    targets: Query<()>,
    game_assets: Res<GameAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    mut rendered: MessageWriter<TileRendered>,
) {
    for request in requests.read() {
        if !targets.contains(request.entity) {
            continue;
        }

        commands.entity(request.entity).insert((
            Visibility::default(),
            Transform::from_translation(request.translation),
        ));

        let mut render = commands.spawn((
            Name::new("HexTileRender"),
            Visibility::default(),
            Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        ));
        let render_entity = render.id();
        render.with_children(|parent| {
            spawn_asset(parent, "HexTileMesh", &game_assets.hex_base, &gltf_assets);
            spawn_tile_decoration(parent, request.kind, &game_assets, &gltf_assets);
        });
        commands.entity(request.entity).add_child(render_entity);
        rendered.write(TileRendered {
            tile_entity: request.entity,
            render_entity,
        });
    }
}

fn spawn_tile_decoration(
    parent: &mut ChildSpawnerCommands,
    kind: TileKind,
    game_assets: &GameAssets,
    gltf_assets: &Assets<Gltf>,
) {
    match kind {
        TileKind::Empty => {}
        TileKind::Tree => spawn_asset(parent, "HexTreeMesh", &game_assets.hex_tree, gltf_assets),
        TileKind::Stone => {
            spawn_asset(parent, "HexStoneMesh", &game_assets.hex_stone, gltf_assets);
        }
        TileKind::Wheat => {
            spawn_asset(parent, "HexDirtMesh", &game_assets.hex_dirt, gltf_assets);
            spawn_asset(parent, "HexWheatMesh", &game_assets.hex_wheat, gltf_assets);
        }
    }
}

fn spawn_asset(
    parent: &mut ChildSpawnerCommands,
    name: &'static str,
    handle: &Handle<Gltf>,
    gltf_assets: &Assets<Gltf>,
) {
    let Some(gltf) = gltf_assets.get(handle) else {
        error!("cannot spawn {name}: GLTF asset is not loaded");
        return;
    };
    let Some(scene) = gltf.scenes.first() else {
        error!("cannot spawn {name}: GLTF asset has no scenes");
        return;
    };
    parent.spawn((
        Name::new(name),
        Transform::default(),
        GlobalTransform::default(),
        WorldAssetRoot(scene.clone()),
    ));
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

    fn test_app() -> App {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<FadingMaterial>>()
            .init_resource::<Assets<Gltf>>()
            .insert_resource(GameAssets {
                hex_base: default(),
                hex_tree: default(),
                hex_stone: default(),
                hex_dirt: default(),
                hex_wheat: default(),
            })
            .add_plugins(ClickerRenderPlugin);
        app
    }

    #[test]
    fn plugin_turns_a_ghost_request_into_render_components() {
        let mut app = test_app();
        let entity = app.world_mut().spawn_empty().id();
        let translation = Vec3::new(1.0, 2.0, 3.0);
        app.world_mut().write_message(RenderGhost {
            entity,
            layout: HexLayout::default(),
            translation,
            depth: 0.5,
        });

        app.update();

        let rendered = app.world().entity(entity);
        assert_eq!(
            rendered
                .get::<Transform>()
                .map(|transform| transform.translation),
            Some(translation)
        );
        assert!(rendered.contains::<Mesh3d>());
        assert!(rendered.contains::<MeshMaterial3d<FadingMaterial>>());
    }

    #[test]
    fn missing_gltf_assets_do_not_block_tile_render_completion() {
        let mut app = test_app();
        let entity = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(RenderTile {
            entity,
            translation: Vec3::ZERO,
            kind: TileKind::Tree,
        });

        app.update();

        assert!(app.world().entity(entity).contains::<Transform>());
        let mut names = app.world_mut().query::<&Name>();
        assert!(names
            .iter(app.world())
            .any(|name| name.as_str() == "HexTileRender"));
        let mut scenes = app.world_mut().query::<&WorldAssetRoot>();
        assert_eq!(scenes.iter(app.world()).count(), 0);
    }

    #[test]
    fn stale_tile_requests_do_not_create_render_data() {
        let mut app = test_app();
        let entity = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(entity).despawn();
        app.world_mut().write_message(RenderTile {
            entity,
            translation: Vec3::ZERO,
            kind: TileKind::Tree,
        });

        app.update();

        let mut names = app.world_mut().query::<&Name>();
        assert_eq!(names.iter(app.world()).count(), 0);
        assert!(app.world().resource::<Messages<TileRendered>>().is_empty());
    }

    #[test]
    fn stale_ghost_requests_do_not_create_render_data() {
        let mut app = test_app();
        let entity = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(entity).despawn();
        app.world_mut().write_message(RenderGhost {
            entity,
            layout: HexLayout::default(),
            translation: Vec3::ZERO,
            depth: 0.5,
        });

        app.update();

        assert!(app.world().resource::<Assets<Mesh>>().is_empty());
        assert!(app.world().resource::<Assets<FadingMaterial>>().is_empty());
    }
}
