use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Resource, Default, Deref, DerefMut)]
struct ViewportHeight(f32);

#[derive(Component, Debug, Default)]
struct CameraOrtho;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraPluginSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // This is the height of the camera's view
        // it will increase to accommodate more tiles
        app.insert_resource(ViewportHeight(6.0));

        app.add_systems(
            Update,
            (setup_camera, update_camera)
                .chain()
                .in_set(CameraPluginSet),
        );
    }
}

fn setup_camera(
    mut commands: Commands,
    q_camera: Query<Entity, (With<Camera3d>, Without<CameraOrtho>)>,
    viewport_height: Res<ViewportHeight>,
) {
    for entity in &q_camera {
        commands.entity(entity).insert((
            CameraOrtho,
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: **viewport_height,
                },
                ..OrthographicProjection::default_3d()
            }),
        ));
    }
}

fn update_camera(
    mut commands: Commands,
    q_camera: Query<Entity, (With<Camera3d>, With<Projection>)>,
    viewport_height: Res<ViewportHeight>,
) {
    if !viewport_height.is_changed() {
        return;
    }

    for entity in &q_camera {
        commands
            .entity(entity)
            .insert(Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: **viewport_height,
                },
                ..OrthographicProjection::default_3d()
            }));
    }
}
