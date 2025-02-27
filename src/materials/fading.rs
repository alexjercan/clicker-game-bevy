use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub mod prelude {
    pub use super::{FadingColor, FadingMaterial, FadingMaterialPlugin, FadingMaterialSet};
}

/// Describe the color that is used to update the fading material
///
/// * When this component changes it will update the material that is used
#[derive(Component, Deref, DerefMut)]
pub struct FadingColor(pub Color);

/// Describe the material that is used on the grid to show textures as tiles
#[derive(Asset, TypePath, Debug, Clone, Default, AsBindGroup)]
pub struct FadingMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

/// System set for the fading material plugin
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FadingMaterialSet;

/// Plugin to handle fading material
pub struct FadingMaterialPlugin;

impl Plugin for FadingMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<FadingMaterial>::default());
        // .add_systems(Update, update_material.in_set(FadingMaterialSet));
    }
}

// fn update_material(
//     q_material: Query<(&Handle<FadingMaterial>, &FadingColor), Changed<FadingColor>>,
//     mut materials: ResMut<Assets<FadingMaterial>>,
// ) {
//     for (fading, FadingColor(color)) in &q_material {
//         if let Some(material) = materials.get_mut(fading) {
//             material.color = *color;
//         }
//     }
// }

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
