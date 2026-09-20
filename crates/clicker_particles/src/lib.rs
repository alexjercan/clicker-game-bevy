use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParticleSettings {
    pub enabled: bool,
}

impl Default for ParticleSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticlePalette {
    Grass,
    Leaves,
    Stone,
    Wheat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleBurstKind {
    TileClick(ParticlePalette),
    TileSpawn,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ParticleBurst {
    pub kind: ParticleBurstKind,
    pub position: Vec3,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleSystems {
    Spawn,
    Cleanup,
}

#[derive(Resource)]
struct ParticleEffects {
    grass_click: Handle<EffectAsset>,
    leaves_click: Handle<EffectAsset>,
    stone_click: Handle<EffectAsset>,
    wheat_click: Handle<EffectAsset>,
    tile_spawn_ring: Handle<EffectAsset>,
    tile_spawn_rise: Handle<EffectAsset>,
}

impl FromWorld for ParticleEffects {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            grass_click: effects.add(burst_effect(BurstEffect {
                name: "grass_click",
                count: 9,
                lifetime: 0.24,
                lifetime_variation: 0.18,
                spawn_radius: 0.0,
                spawn_radius_variation: 0.08,
                horizontal_speed: 1.2,
                horizontal_variation: 2.4,
                vertical_speed: 1.2,
                vertical_variation: 2.5,
                gravity: -7.0,
                drag: 3.0,
                color_start: Vec4::new(0.7, 1.0, 0.36, 1.0),
                color_end: Vec4::new(0.18, 0.55, 0.16, 0.0),
                size: Vec3::new(0.075, 0.075, 1.0),
                roundness: 1.0,
                orient_along_velocity: false,
            })),
            leaves_click: effects.add(burst_effect(BurstEffect {
                name: "leaves_click",
                count: 12,
                lifetime: 0.3,
                lifetime_variation: 0.22,
                spawn_radius: 0.0,
                spawn_radius_variation: 0.08,
                horizontal_speed: 1.0,
                horizontal_variation: 2.0,
                vertical_speed: 2.5,
                vertical_variation: 3.5,
                gravity: -8.0,
                drag: 2.5,
                color_start: Vec4::new(0.35, 1.0, 0.32, 1.0),
                color_end: Vec4::new(0.04, 0.28, 0.12, 0.0),
                size: Vec3::new(0.045, 0.14, 1.0),
                roundness: 0.7,
                orient_along_velocity: true,
            })),
            stone_click: effects.add(burst_effect(BurstEffect {
                name: "stone_click",
                count: 10,
                lifetime: 0.2,
                lifetime_variation: 0.18,
                spawn_radius: 0.0,
                spawn_radius_variation: 0.07,
                horizontal_speed: 2.0,
                horizontal_variation: 3.0,
                vertical_speed: 1.0,
                vertical_variation: 2.5,
                gravity: -14.0,
                drag: 2.0,
                color_start: Vec4::new(0.9, 0.95, 1.0, 1.0),
                color_end: Vec4::new(0.32, 0.38, 0.45, 0.0),
                size: Vec3::new(0.08, 0.065, 1.0),
                roundness: 0.12,
                orient_along_velocity: false,
            })),
            wheat_click: effects.add(burst_effect(BurstEffect {
                name: "wheat_click",
                count: 13,
                lifetime: 0.34,
                lifetime_variation: 0.22,
                spawn_radius: 0.0,
                spawn_radius_variation: 0.08,
                horizontal_speed: 1.2,
                horizontal_variation: 2.4,
                vertical_speed: 2.2,
                vertical_variation: 3.8,
                gravity: -6.0,
                drag: 3.5,
                color_start: Vec4::new(1.0, 0.9, 0.28, 1.0),
                color_end: Vec4::new(0.78, 0.38, 0.05, 0.0),
                size: Vec3::new(0.035, 0.15, 1.0),
                roundness: 0.55,
                orient_along_velocity: true,
            })),
            tile_spawn_ring: effects.add(burst_effect(BurstEffect {
                name: "tile_spawn_ring",
                count: 22,
                lifetime: 0.34,
                lifetime_variation: 0.16,
                spawn_radius: 0.45,
                spawn_radius_variation: 0.3,
                horizontal_speed: 2.5,
                horizontal_variation: 2.5,
                vertical_speed: 1.2,
                vertical_variation: 2.0,
                gravity: -4.0,
                drag: 3.5,
                color_start: Vec4::new(1.0, 0.94, 0.58, 1.0),
                color_end: Vec4::new(0.58, 0.92, 0.3, 0.0),
                size: Vec3::new(0.11, 0.11, 1.0),
                roundness: 1.0,
                orient_along_velocity: false,
            })),
            tile_spawn_rise: effects.add(burst_effect(BurstEffect {
                name: "tile_spawn_rise",
                count: 18,
                lifetime: 0.42,
                lifetime_variation: 0.25,
                spawn_radius: 0.15,
                spawn_radius_variation: 0.5,
                horizontal_speed: 0.8,
                horizontal_variation: 2.2,
                vertical_speed: 4.0,
                vertical_variation: 5.0,
                gravity: -8.0,
                drag: 2.8,
                color_start: Vec4::new(1.0, 0.72, 0.16, 1.0),
                color_end: Vec4::new(1.0, 0.96, 0.55, 0.0),
                size: Vec3::new(0.05, 0.2, 1.0),
                roundness: 0.65,
                orient_along_velocity: true,
            })),
        }
    }
}

#[derive(Component)]
struct ParticleEffectLifetime(Timer);

pub struct ClickerParticlesPlugin;

impl Plugin for ClickerParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .init_resource::<ParticleSettings>()
            .init_resource::<ParticleEffects>()
            .add_message::<ParticleBurst>()
            .configure_sets(
                Update,
                (ParticleSystems::Spawn, ParticleSystems::Cleanup).chain(),
            )
            .add_systems(Update, spawn_bursts.in_set(ParticleSystems::Spawn))
            .add_systems(
                Update,
                cleanup_finished_effects.in_set(ParticleSystems::Cleanup),
            );
    }
}

struct BurstEffect {
    name: &'static str,
    count: u32,
    lifetime: f32,
    lifetime_variation: f32,
    spawn_radius: f32,
    spawn_radius_variation: f32,
    horizontal_speed: f32,
    horizontal_variation: f32,
    vertical_speed: f32,
    vertical_variation: f32,
    gravity: f32,
    drag: f32,
    color_start: Vec4,
    color_end: Vec4,
    size: Vec3,
    roundness: f32,
    orient_along_velocity: bool,
}

fn burst_effect(config: BurstEffect) -> EffectAsset {
    let writer = ExprWriter::new();
    let angle = writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU);
    let horizontal = writer.lit(config.horizontal_speed)
        + writer.rand(ScalarType::Float) * writer.lit(config.horizontal_variation);
    let vertical = writer.lit(config.vertical_speed)
        + writer.rand(ScalarType::Float) * writer.lit(config.vertical_variation);
    let velocity = (angle.clone().cos() * horizontal.clone())
        .vec3(vertical, angle.clone().sin() * horizontal)
        .expr();
    let radius = writer.lit(config.spawn_radius)
        + writer.rand(ScalarType::Float) * writer.lit(config.spawn_radius_variation);
    let position = (angle.clone().cos() * radius.clone())
        .vec3(writer.lit(0.0), angle.sin() * radius)
        .expr();
    let particle_lifetime = (writer.lit(config.lifetime)
        + writer.rand(ScalarType::Float) * writer.lit(config.lifetime_variation))
    .expr();
    let drag = writer.lit(config.drag).expr();
    let gravity = writer.lit(Vec3::Y * config.gravity).expr();
    let mut module = writer.finish();
    let round = RoundModifier::constant(&mut module, config.roundness);

    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, config.color_start);
    color_gradient.add_key(0.65, config.color_start);
    color_gradient.add_key(1.0, config.color_end);

    let mut size_gradient = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, config.size * 0.35);
    size_gradient.add_key(0.12, config.size);
    size_gradient.add_key(0.72, config.size * 0.8);
    size_gradient.add_key(1.0, Vec3::ZERO);

    let mut effect = EffectAsset::new(
        config.count,
        SpawnerSettings::once((config.count as f32).into()),
        module,
    )
    .with_name(config.name)
    .init(SetAttributeModifier::new(Attribute::POSITION, position))
    .init(SetAttributeModifier::new(Attribute::VELOCITY, velocity))
    .init(SetAttributeModifier::new(
        Attribute::LIFETIME,
        particle_lifetime,
    ))
    .update(LinearDragModifier::new(drag))
    .update(AccelModifier::new(gravity))
    .render(ColorOverLifetimeModifier::new(color_gradient))
    .render(SizeOverLifetimeModifier {
        gradient: size_gradient,
        screen_space_size: false,
    })
    .render(round);
    if config.orient_along_velocity {
        effect = effect.render(OrientModifier::new(OrientMode::AlongVelocity));
    }
    effect
}

fn spawn_bursts(
    mut commands: Commands,
    settings: Res<ParticleSettings>,
    effects: Res<ParticleEffects>,
    mut bursts: MessageReader<ParticleBurst>,
) {
    for burst in bursts.read() {
        if !settings.enabled {
            continue;
        }
        match burst.kind {
            ParticleBurstKind::TileClick(palette) => spawn_effect(
                &mut commands,
                "Particles_TileClick",
                click_effect(&effects, palette),
                burst.position,
                0.65,
            ),
            ParticleBurstKind::TileSpawn => {
                spawn_effect(
                    &mut commands,
                    "Particles_TileSpawnRing",
                    &effects.tile_spawn_ring,
                    burst.position,
                    0.7,
                );
                spawn_effect(
                    &mut commands,
                    "Particles_TileSpawnRise",
                    &effects.tile_spawn_rise,
                    burst.position,
                    0.9,
                );
            }
        }
    }
}

fn click_effect(effects: &ParticleEffects, palette: ParticlePalette) -> &Handle<EffectAsset> {
    match palette {
        ParticlePalette::Grass => &effects.grass_click,
        ParticlePalette::Leaves => &effects.leaves_click,
        ParticlePalette::Stone => &effects.stone_click,
        ParticlePalette::Wheat => &effects.wheat_click,
    }
}

fn spawn_effect(
    commands: &mut Commands,
    name: &'static str,
    effect: &Handle<EffectAsset>,
    position: Vec3,
    lifetime: f32,
) {
    commands.spawn((
        Name::new(name),
        ParticleEffect::new(effect.clone()),
        Transform::from_translation(position),
        ParticleEffectLifetime(Timer::from_seconds(lifetime, TimerMode::Once)),
    ));
}

fn cleanup_finished_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut ParticleEffectLifetime)>,
) {
    for (entity, mut lifetime) in &mut effects {
        if lifetime.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_particles_consume_bursts_without_spawning_effects() {
        let mut app = App::new();
        app.add_message::<ParticleBurst>()
            .insert_resource(ParticleSettings { enabled: false })
            .insert_resource(ParticleEffects {
                grass_click: default(),
                leaves_click: default(),
                stone_click: default(),
                wheat_click: default(),
                tile_spawn_ring: default(),
                tile_spawn_rise: default(),
            })
            .add_systems(Update, spawn_bursts);
        app.world_mut().write_message(ParticleBurst {
            kind: ParticleBurstKind::TileClick(ParticlePalette::Stone),
            position: Vec3::ZERO,
        });

        app.update();

        let mut effects = app.world_mut().query::<&ParticleEffect>();
        assert_eq!(effects.iter(app.world()).count(), 0);
    }
}
