mod audio;
mod camera;
mod tile;

use bevy::prelude::*;

pub(crate) struct RenderedGluePlugin;

impl Plugin for RenderedGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            tile::RenderedTileGluePlugin,
            audio::AudioGluePlugin,
            camera::CameraGluePlugin,
        ));
    }
}

pub(crate) struct HeadlessGluePlugin;

impl Plugin for HeadlessGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(tile::HeadlessTileGluePlugin);
    }
}
