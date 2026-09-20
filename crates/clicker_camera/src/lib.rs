use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct CameraShakeSettings {
    pub enabled: bool,
    pub click_impulse: f32,
    pub maximum_amplitude: f32,
    pub decay_per_second: f32,
    pub frequency_hz: f32,
}

impl Default for CameraShakeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            click_impulse: 0.035,
            maximum_amplitude: 0.08,
            decay_per_second: 0.25,
            frequency_hz: 20.0,
        }
    }
}

// NOTE: Shake owns the local transform of its entity, so it must sit on a camera child whose parent holds the camera pose.
#[derive(Component, Debug, Default)]
pub struct ShakeCamera {
    amplitude: f32,
    phase: f32,
}

impl ShakeCamera {
    fn add_impulse(&mut self, strength: f32, settings: CameraShakeSettings) {
        if settings.enabled {
            self.amplitude = (self.amplitude + strength).min(settings.maximum_amplitude);
        }
    }

    fn decay(&mut self, seconds: f32, settings: CameraShakeSettings) {
        self.amplitude = (self.amplitude - settings.decay_per_second * seconds).max(0.0);
    }

    fn offset(&self) -> Vec3 {
        if self.amplitude == 0.0 {
            return Vec3::ZERO;
        }

        Vec3::new(self.phase.sin(), (self.phase * 1.7).cos(), 0.0).normalize_or_zero()
            * self.amplitude
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct CameraImpulse {
    pub strength: f32,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraFeedbackSystems {
    Impulse,
    Motion,
}

pub struct ClickerCameraPlugin;

impl Plugin for ClickerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraShakeSettings>()
            .add_message::<CameraImpulse>()
            .configure_sets(
                Update,
                (
                    CameraFeedbackSystems::Impulse,
                    CameraFeedbackSystems::Motion,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                apply_camera_impulses.in_set(CameraFeedbackSystems::Impulse),
            )
            .add_systems(
                Update,
                update_camera_shake.in_set(CameraFeedbackSystems::Motion),
            );
    }
}

fn apply_camera_impulses(
    settings: Res<CameraShakeSettings>,
    mut impulses: MessageReader<CameraImpulse>,
    mut cameras: Query<&mut ShakeCamera>,
) {
    let strength = impulses.read().map(|impulse| impulse.strength).sum();
    for mut camera in &mut cameras {
        camera.add_impulse(strength, *settings);
    }
}

fn update_camera_shake(
    time: Res<Time>,
    settings: Res<CameraShakeSettings>,
    mut cameras: Query<(&mut Transform, &mut ShakeCamera)>,
) {
    for (mut transform, mut shake) in &mut cameras {
        if settings.enabled {
            let seconds = time.delta_secs();
            shake.phase += seconds * settings.frequency_hz * std::f32::consts::TAU;
            shake.decay(seconds, *settings);
        } else {
            shake.amplitude = 0.0;
        }

        let offset = shake.offset();
        if transform.translation != offset {
            transform.translation = offset;
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::transform::TransformPlugin;

    use super::*;

    #[derive(Component)]
    struct CameraRig;

    fn shake_app(settings: CameraShakeSettings) -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .insert_resource(settings)
            .add_plugins((TransformPlugin, ClickerCameraPlugin));
        app
    }

    fn spawn_camera_rig(app: &mut App, pose: Transform) -> Entity {
        app.world_mut()
            .spawn((CameraRig, pose))
            .with_child((Transform::default(), ShakeCamera::default()))
            .id()
    }

    fn local_translation(app: &App, entity: Entity) -> Vec3 {
        app.world()
            .entity(entity)
            .get::<Transform>()
            .unwrap()
            .translation
    }

    fn global_translation(app: &App, entity: Entity) -> Vec3 {
        app.world()
            .entity(entity)
            .get::<GlobalTransform>()
            .unwrap()
            .translation()
    }

    fn shake_entity(app: &mut App) -> Entity {
        let mut cameras = app
            .world_mut()
            .query_filtered::<Entity, With<ShakeCamera>>();
        cameras.single(app.world()).unwrap()
    }

    #[test]
    fn impulses_accumulate_up_to_the_maximum() {
        let settings = CameraShakeSettings::default();
        let mut shake = ShakeCamera::default();

        shake.add_impulse(settings.click_impulse, settings);
        shake.add_impulse(settings.click_impulse, settings);
        assert_eq!(shake.amplitude, settings.click_impulse * 2.0);

        shake.add_impulse(settings.maximum_amplitude, settings);
        assert_eq!(shake.amplitude, settings.maximum_amplitude);
    }

    #[test]
    fn amplitude_decays_to_zero() {
        let settings = CameraShakeSettings::default();
        let mut shake = ShakeCamera::default();
        shake.add_impulse(settings.click_impulse, settings);

        shake.decay(0.06, settings);
        assert!((shake.amplitude - 0.02).abs() < f32::EPSILON);

        shake.decay(1.0, settings);
        assert_eq!(shake.amplitude, 0.0);
    }

    #[test]
    fn disabled_shake_ignores_impulses() {
        let settings = CameraShakeSettings {
            enabled: false,
            ..default()
        };
        let mut shake = ShakeCamera::default();

        shake.add_impulse(0.1, settings);
        assert_eq!(shake.amplitude, 0.0);
    }

    #[test]
    fn plugin_applies_replaces_and_clears_camera_offset() {
        let mut app = shake_app(CameraShakeSettings {
            decay_per_second: 0.0,
            frequency_hz: 0.0,
            ..default()
        });
        spawn_camera_rig(&mut app, Transform::default());
        let camera = shake_entity(&mut app);
        app.world_mut()
            .write_message(CameraImpulse { strength: 0.05 });

        app.update();
        assert_eq!(local_translation(&app, camera), Vec3::new(0.0, 0.05, 0.0));

        app.update();
        assert_eq!(local_translation(&app, camera), Vec3::new(0.0, 0.05, 0.0));

        app.world_mut()
            .resource_mut::<CameraShakeSettings>()
            .enabled = false;
        app.update();
        assert_eq!(local_translation(&app, camera), Vec3::ZERO);
    }

    #[test]
    fn shake_preserves_a_camera_pose_written_by_another_system() {
        let mut app = shake_app(CameraShakeSettings {
            decay_per_second: 0.0,
            frequency_hz: 0.0,
            ..default()
        });
        app.add_systems(Update, pan_camera_rig.after(CameraFeedbackSystems::Motion));
        let rig = spawn_camera_rig(&mut app, Transform::default());
        let camera = shake_entity(&mut app);
        app.world_mut()
            .write_message(CameraImpulse { strength: 0.05 });

        app.update();
        app.update();

        let offset = Vec3::new(0.0, 0.05, 0.0);
        assert_eq!(local_translation(&app, rig), Vec3::new(2.0, 0.0, 0.0));
        assert_eq!(local_translation(&app, camera), offset);
        assert_eq!(
            global_translation(&app, camera),
            Vec3::new(2.0, 0.0, 0.0) + offset
        );

        app.world_mut()
            .resource_mut::<CameraShakeSettings>()
            .enabled = false;
        app.update();
        assert_eq!(local_translation(&app, rig), Vec3::new(3.0, 0.0, 0.0));
        assert_eq!(local_translation(&app, camera), Vec3::ZERO);
        assert_eq!(global_translation(&app, camera), Vec3::new(3.0, 0.0, 0.0));
    }

    #[test]
    fn shake_offset_stays_in_camera_space() {
        let mut app = shake_app(CameraShakeSettings {
            decay_per_second: 0.0,
            frequency_hz: 0.0,
            ..default()
        });
        let pose = Transform::from_xyz(0.0, 15.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y);
        spawn_camera_rig(&mut app, pose);
        let camera = shake_entity(&mut app);
        app.world_mut()
            .write_message(CameraImpulse { strength: 0.05 });

        app.update();

        let global = global_translation(&app, camera);
        let expected = pose.translation + pose.rotation * Vec3::new(0.0, 0.05, 0.0);
        assert!(global.abs_diff_eq(expected, 1e-5));
    }

    fn pan_camera_rig(mut rigs: Query<&mut Transform, With<CameraRig>>, mut frame: Local<f32>) {
        *frame += 1.0;
        for mut rig in &mut rigs {
            rig.translation = Vec3::X * *frame;
        }
    }
}
