mod materials;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use clicker_state::GameState;

pub use materials::prelude::{FadingMaterial, FadingMaterialPlugin};

pub const BACKGROUND_DARK_COLOR: Color = Color::srgb(0.65, 0.65, 0.65);

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "undefined.png")]
    pub skill_point: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "gltf/tiles/base/hex_grass.gltf")]
    pub hex_base: Handle<Gltf>,
    #[asset(path = "gltf/decoration/nature/trees_A_large.gltf")]
    pub hex_tree: Handle<Gltf>,
    #[asset(path = "gltf/decoration/nature/hills_A.gltf")]
    pub hex_stone: Handle<Gltf>,
    #[asset(path = "gltf/buildings/neutral/building_dirt.gltf")]
    pub hex_dirt: Handle<Gltf>,
    #[asset(path = "gltf/buildings/neutral/building_grain.gltf")]
    pub hex_wheat: Handle<Gltf>,
}

pub struct ClickerAssetsPlugin;

impl Plugin for ClickerAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FadingMaterialPlugin)
            .init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Playing)
                    .load_collection::<UiAssets>()
                    .load_collection::<GameAssets>(),
            )
            .add_systems(OnEnter(GameState::AssetLoading), setup_asset_loading);
    }
}

fn setup_asset_loading(mut commands: Commands) {
    commands.spawn((
        Name::new("CameraUI"),
        Camera2d,
        DespawnOnExit(GameState::AssetLoading),
    ));
}
