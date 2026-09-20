mod audio;
mod camera;
mod particles;
mod render;
mod tile;

use bevy::prelude::*;

pub(crate) struct RenderedGluePlugin;

impl Plugin for RenderedGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            tile::TileGluePlugin,
            render::RenderGluePlugin,
            audio::AudioGluePlugin,
            camera::CameraGluePlugin,
            particles::ParticleGluePlugin,
        ));
    }
}

pub(crate) struct HeadlessGluePlugin;

impl Plugin for HeadlessGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(tile::TileGluePlugin);
    }
}
