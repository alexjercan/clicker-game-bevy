//! Camera plugin for Bevy that provides a simple orthographic camera with smooth zooming.

use bevy::{prelude::*, render::camera::ScalingMode};

use crate::meth::*;

/// Target value for the camera's view height. When this value is changed the camera will smoothly
/// match the new value.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct ViewportHeight(pub f32);

#[derive(Component, Debug)]
pub struct CameraOrtho {
    /// The smoothing factor for the camera's view height
    pub smoothing: f32,
    /// Initial viewport height of the camera
    pub viewport_height: f32,
}

impl Default for CameraOrtho {
    fn default() -> Self {
        Self {
            smoothing: 0.1,
            viewport_height: 6.0,
        }
    }
}

#[derive(Component, Debug)]
struct CameraOrthoState {
    pub viewport_height: f32,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraPluginSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ViewportHeight(6.0));

        app.add_systems(Update, setup_camera.in_set(CameraPluginSet));
        app.add_systems(Update, (update_camera_state, update_camera).chain().in_set(CameraPluginSet));
    }
}

fn setup_camera(
    mut commands: Commands,
    q_camera: Query<(Entity, &CameraOrtho), Without<CameraOrthoState>>,
) {
    for (entity, CameraOrtho { viewport_height, .. }) in &q_camera {
        commands
            .entity(entity)
            .insert(CameraOrthoState {
                viewport_height: *viewport_height,
            });
    }
}

fn update_camera_state(
    time: Res<Time>,
    mut q_camera: Query<(&CameraOrtho, &mut CameraOrthoState)>,
    viewport_height: Res<ViewportHeight>,
) {
    if !viewport_height.is_changed() {
        return;
    }

    for (CameraOrtho { smoothing, .. }, mut state) in q_camera.iter_mut() {
        state.viewport_height = state.viewport_height.lerp_and_snap(**viewport_height, *smoothing, time.delta_secs());
    }
}

fn update_camera(
    mut commands: Commands,
    q_camera: Query<(Entity, &CameraOrthoState)>,
) {
    for (entity, state) in &q_camera {
        commands
            .entity(entity)
            .insert(Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: state.viewport_height,
                },
                ..OrthographicProjection::default_3d()
            }));
    }
}
