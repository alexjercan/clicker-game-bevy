use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowMode};
use bevy::{
    log::{BoxedLayer, LogPlugin},
    state::app::StatesPlugin,
};
use clicker_animation::ClickerAnimationPlugin;
use clicker_assets::ClickerAssetsPlugin;
use clicker_gameplay::ClickerGameplayPlugin;
use clicker_state::GameState;
use clicker_ui::ClickerUiPlugin;

use crate::{
    glue::{HeadlessTileGluePlugin, RenderedTileGluePlugin},
    scene::MainScenePlugin,
};

pub const NORENDER_ENV: &str = "CLICKER_NORENDER";

#[cfg(all(feature = "dev", not(target_arch = "wasm32")))]
#[derive(Resource)]
struct ChromeTraceGuard {
    _guard: std::sync::Mutex<tracing_chrome::FlushGuard>,
}

fn chrome_trace_layer(app: &mut App) -> Option<BoxedLayer> {
    #[cfg(all(feature = "dev", not(target_arch = "wasm32")))]
    {
        let path = std::env::var("TRACE_CHROME").ok()?;
        let (layer, guard) = tracing_chrome::ChromeLayerBuilder::new()
            .file(path)
            .name_fn(Box::new(|event_or_span| match event_or_span {
                tracing_chrome::EventOrSpan::Event(event) => event.metadata().name().into(),
                tracing_chrome::EventOrSpan::Span(span) => span
                    .extensions()
                    .get::<tracing_subscriber::fmt::FormattedFields<
                        tracing_subscriber::fmt::format::DefaultFields,
                    >>()
                    .map_or_else(
                        || span.metadata().name().into(),
                        |fields| format!("{}: {}", span.metadata().name(), fields.fields),
                    ),
            }))
            .build();
        app.insert_resource(ChromeTraceGuard {
            _guard: std::sync::Mutex::new(guard),
        });
        Some(Box::new(layer))
    }
    #[cfg(not(all(feature = "dev", not(target_arch = "wasm32"))))]
    {
        let _ = app;
        None
    }
}

fn log_plugin() -> LogPlugin {
    LogPlugin {
        custom_layer: chrome_trace_layer,
        ..default()
    }
}

pub fn app() -> App {
    if std::env::var_os(NORENDER_ENV).is_some() {
        headless_app()
    } else {
        rendered_app()
    }
}

fn rendered_app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(log_plugin())
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
    )
    .add_plugins((
        ClickerAnimationPlugin,
        ClickerAssetsPlugin,
        MainScenePlugin,
        ClickerGameplayPlugin,
        RenderedTileGluePlugin,
        ClickerUiPlugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(clicker_debug::DebugPlugin);

    app
}

pub fn headless_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, log_plugin()));
    app.init_state::<GameState>()
        .add_plugins((ClickerGameplayPlugin, HeadlessTileGluePlugin))
        .add_systems(Startup, enter_playing);
    app
}

fn enter_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

#[cfg(test)]
mod tests {
    use bevy::render::RenderApp;
    use clicker_gameplay::{ClickTile, HexTile, SkillPoints, TileCoord, TilePointer, XpValue};

    use super::*;

    #[test]
    fn headless_app_runs_gameplay_without_rendering() {
        let mut app = headless_app();
        app.update();
        assert!(app.get_sub_app(RenderApp).is_none());
        let mut windows = app.world_mut().query::<&Window>();
        assert_eq!(windows.iter(app.world()).count(), 0);
        app.world_mut().resource_mut::<TilePointer>().world_position = Some(Vec2::ZERO);
        app.world_mut().resource_mut::<TilePointer>().activate = true;
        app.update();
        app.world_mut().resource_mut::<TilePointer>().activate = false;
        app.world_mut().write_message(ClickTile(TileCoord::ZERO));
        app.update();
        let mut tiles = app.world_mut().query_filtered::<Entity, With<HexTile>>();
        assert_eq!(tiles.iter(app.world()).count(), 1);
        assert_eq!(app.world().resource::<XpValue>().0, 1);
        assert_eq!(app.world().resource::<SkillPoints>().0, 0);
    }
}
