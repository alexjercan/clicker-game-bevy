use bevy::{audio::Volume, prelude::*};
use clicker_assets::SfxAssets;
use clicker_tile::TileKind;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct SfxSettings {
    pub enabled: bool,
    pub volume: f32,
}

impl Default for SfxSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            volume: 0.65,
        }
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfxCue {
    TileClick(TileKind),
    Selection,
    Placement,
    LevelUp,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioSystems {
    Playback,
}

#[derive(Component)]
struct SfxPlayer;

pub struct ClickerAudioPlugin;

impl Plugin for ClickerAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SfxSettings>()
            .add_message::<SfxCue>()
            .add_systems(
                Update,
                play_sfx
                    .in_set(AudioSystems::Playback)
                    .run_if(resource_exists::<SfxAssets>),
            )
            .add_systems(Last, despawn_unplayed_sfx);
    }
}

fn play_sfx(
    mut commands: Commands,
    settings: Res<SfxSettings>,
    assets: Res<SfxAssets>,
    mut cues: MessageReader<SfxCue>,
) {
    for cue in cues.read() {
        if !settings.enabled {
            continue;
        }
        let (name, source) = match cue {
            SfxCue::TileClick(TileKind::Empty) => ("Sfx_ClickEmpty", &assets.click_empty),
            SfxCue::TileClick(TileKind::Tree) => ("Sfx_ClickTree", &assets.click_tree),
            SfxCue::TileClick(TileKind::Stone) => ("Sfx_ClickStone", &assets.click_stone),
            SfxCue::TileClick(TileKind::Wheat) => ("Sfx_ClickWheat", &assets.click_wheat),
            SfxCue::Selection => ("Sfx_Selection", &assets.select),
            SfxCue::Placement => ("Sfx_Placement", &assets.place),
            SfxCue::LevelUp => ("Sfx_LevelUp", &assets.level_up),
        };
        commands.spawn((
            Name::new(name),
            SfxPlayer,
            AudioPlayer::new(source.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.volume)),
        ));
    }
}

// NOTE: Bevy skips its audio playback systems without an output stream, so a cue
// NOTE: never gets an `AudioSink` and `PlaybackSettings::DESPAWN` never removes it.
// NOTE: Bevy inserts the sink in `PostUpdate`, so a cue without one here cannot play.
fn despawn_unplayed_sfx(
    mut commands: Commands,
    unplayed: Query<Entity, (With<SfxPlayer>, Without<AudioSink>)>,
) {
    for entity in &unplayed {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_waits_for_loaded_sfx_assets() {
        let mut app = App::new();
        app.add_message::<SfxCue>()
            .init_resource::<SfxSettings>()
            .add_systems(Update, play_sfx.run_if(resource_exists::<SfxAssets>));
        app.world_mut().write_message(SfxCue::LevelUp);

        app.update();

        let mut players = app.world_mut().query::<&AudioPlayer>();
        assert_eq!(players.iter(app.world()).count(), 0);
    }

    #[test]
    fn disabled_sfx_consumes_cues_without_spawning_players() {
        let mut app = App::new();
        app.add_message::<SfxCue>()
            .insert_resource(SfxSettings {
                enabled: false,
                ..default()
            })
            .insert_resource(loaded_sfx_assets())
            .add_systems(Update, play_sfx);
        app.world_mut().write_message(SfxCue::LevelUp);

        app.update();

        let mut players = app.world_mut().query::<&AudioPlayer>();
        assert_eq!(players.iter(app.world()).count(), 0);
    }

    #[test]
    fn cues_without_audio_output_do_not_leak_players() {
        let mut app = App::new();
        app.add_plugins(ClickerAudioPlugin)
            .insert_resource(loaded_sfx_assets())
            .init_resource::<ObservedPlayers>()
            .add_systems(PostUpdate, observe_players);

        for _ in 0..4 {
            app.world_mut().write_message(SfxCue::LevelUp);
            app.update();
        }

        assert_eq!(app.world().resource::<ObservedPlayers>().0, 1);
        let mut players = app.world_mut().query::<&AudioPlayer>();
        assert_eq!(players.iter(app.world()).count(), 0);
    }

    #[test]
    fn playing_sfx_players_stay_until_bevy_despawns_them() {
        let mut app = App::new();
        app.add_plugins(ClickerAudioPlugin)
            .insert_resource(loaded_sfx_assets());
        let (player, _output) = rodio::Player::new();
        let entity = app
            .world_mut()
            .spawn((
                SfxPlayer,
                AudioPlayer::new(Handle::<AudioSource>::default()),
                PlaybackSettings::DESPAWN,
                AudioSink::new(player),
            ))
            .id();

        app.update();

        assert!(app.world().get_entity(entity).is_ok());
    }

    #[derive(Resource, Default)]
    struct ObservedPlayers(usize);

    fn observe_players(
        players: Query<(), With<AudioPlayer>>,
        mut observed: ResMut<ObservedPlayers>,
    ) {
        observed.0 = observed.0.max(players.iter().count());
    }

    fn loaded_sfx_assets() -> SfxAssets {
        SfxAssets {
            click_empty: default(),
            click_tree: default(),
            click_stone: default(),
            click_wheat: default(),
            select: default(),
            place: default(),
            level_up: default(),
        }
    }
}
