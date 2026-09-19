use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

pub mod prelude {
    pub use super::{FadingMaterial, FadingMaterialPlugin};
}

#[derive(Asset, TypePath, Debug, Clone, Default, AsBindGroup)]
pub struct FadingMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

pub struct FadingMaterialPlugin;

impl Plugin for FadingMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<FadingMaterial>::default());
    }
}

impl FadingMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            color: color.into(),
        }
    }
}

impl Material for FadingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fading.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
