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

#[derive(Component, Debug, Default)]
pub struct ShakeCamera {
    amplitude: f32,
    phase: f32,
    applied_offset: Vec3,
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

    fn disable(&mut self) {
        self.amplitude = 0.0;
        self.applied_offset = Vec3::ZERO;
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
        transform.translation -= shake.applied_offset;
        shake.applied_offset = Vec3::ZERO;

        if !settings.enabled {
            shake.disable();
            continue;
        }

        let seconds = time.delta_secs();
        shake.phase += seconds * settings.frequency_hz * std::f32::consts::TAU;
        shake.decay(seconds, *settings);
        if shake.amplitude == 0.0 {
            continue;
        }

        let screen_offset = Vec3::new(shake.phase.sin(), (shake.phase * 1.7).cos(), 0.0)
            .normalize_or_zero()
            * shake.amplitude;
        shake.applied_offset = transform.rotation * screen_offset;
        transform.translation += shake.applied_offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn plugin_applies_replaces_and_clears_camera_offset() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .insert_resource(CameraShakeSettings {
                decay_per_second: 0.0,
                frequency_hz: 0.0,
                ..default()
            })
            .add_plugins(ClickerCameraPlugin);
        let camera = app
            .world_mut()
            .spawn((Transform::default(), ShakeCamera::default()))
            .id();
        app.world_mut()
            .write_message(CameraImpulse { strength: 0.05 });

        app.update();
        let first_translation = app
            .world()
            .entity(camera)
            .get::<Transform>()
            .unwrap()
            .translation;
        assert_eq!(first_translation, Vec3::new(0.0, 0.05, 0.0));

        app.update();
        let second_translation = app
            .world()
            .entity(camera)
            .get::<Transform>()
            .unwrap()
            .translation;
        assert_eq!(second_translation, first_translation);

        app.world_mut()
            .resource_mut::<CameraShakeSettings>()
            .enabled = false;
        app.update();
        let disabled_translation = app
            .world()
            .entity(camera)
            .get::<Transform>()
            .unwrap()
            .translation;
        assert_eq!(disabled_translation, Vec3::ZERO);
    }

    #[test]
    fn disabled_shake_ignores_impulses_and_clears_motion() {
        let settings = CameraShakeSettings {
            enabled: false,
            ..default()
        };
        let mut shake = ShakeCamera {
            amplitude: 0.1,
            applied_offset: Vec3::ONE,
            ..default()
        };

        shake.add_impulse(0.1, settings);
        assert_eq!(shake.amplitude, 0.1);
        shake.disable();
        assert_eq!(shake.amplitude, 0.0);
        assert_eq!(shake.applied_offset, Vec3::ZERO);
    }
}
