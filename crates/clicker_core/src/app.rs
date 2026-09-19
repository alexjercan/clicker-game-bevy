use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowMode};
use clicker_animation::ClickerAnimationPlugin;
use clicker_assets::ClickerAssetsPlugin;
use clicker_gameplay::ClickerGameplayPlugin;
use clicker_ui::ClickerUiPlugin;

pub fn app() -> App {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Clicker".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins((
        ClickerAnimationPlugin,
        ClickerAssetsPlugin,
        ClickerGameplayPlugin,
        ClickerUiPlugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(clicker_debug::DebugPlugin);

    app
}
