//! The game core module. This module will contain the game's core logic and systems.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[cfg(feature = "debug")]
use crate::debug::*;

use self::{light::*, render::*, tweening::*, ui::*};
use crate::{camera::*, hex::*, levelxp::*, skilltree::*};

mod light;
mod render;
mod tweening;
mod ui;

pub use self::ui::RootUI;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    #[default]
    AssetLoading,
    Playing,
}

#[derive(AssetCollection, Resource)]
pub struct UIAssets {
    #[asset(path = "undefined.png")]
    pub skill_tree_points_alert: Handle<Image>,
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

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
        app.add_plugins(LevelXPPlugin);
        app.add_plugins(SkillTreePlugin);
        app.add_plugins(HexPlugin);

        app.add_plugins(LightPlugin);
        app.add_plugins(RenderPlugin);
        app.add_plugins(TweeningPlugin);
        app.add_plugins(UIPlugin);

        #[cfg(feature = "debug")]
        app.add_plugins(DebugPlugin);

        app.init_state::<GameStates>();
        app.enable_state_scoped_entities::<GameStates>();

        app.add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Playing)
                .load_collection::<UIAssets>()
                .load_collection::<GameAssets>(),
        );

        app.add_systems(OnEnter(GameStates::AssetLoading), setup_asset_loading);
        app.add_systems(OnEnter(GameStates::Playing), setup_playing);

        app.configure_sets(
            Update,
            CameraPluginSet.run_if(in_state(GameStates::Playing)),
        );
        app.configure_sets(
            Update,
            LevelXPPluginSet.run_if(in_state(GameStates::Playing)),
        );
        app.configure_sets(
            Update,
            SkillTreePluginSet.run_if(in_state(GameStates::Playing)),
        );
        app.configure_sets(Update, HexPluginSet.run_if(in_state(GameStates::Playing)));
    }
}

fn setup_asset_loading(mut commands: Commands) {
    commands.spawn((
        Name::new("CameraUI"),
        Camera2d::default(),
        StateScoped(GameStates::AssetLoading),
    ));
}

fn setup_playing(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera3D"),
        Camera3d::default(),
        Transform::from_xyz(-15.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraOrtho::default(),
        StateScoped(GameStates::Playing),
    ));
}
