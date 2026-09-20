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
            );
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
            AudioPlayer::new(source.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.volume)),
        ));
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
            .insert_resource(SfxAssets {
                click_empty: default(),
                click_tree: default(),
                click_stone: default(),
                click_wheat: default(),
                select: default(),
                place: default(),
                level_up: default(),
            })
            .add_systems(Update, play_sfx);
        app.world_mut().write_message(SfxCue::LevelUp);

        app.update();

        let mut players = app.world_mut().query::<&AudioPlayer>();
        assert_eq!(players.iter(app.world()).count(), 0);
    }
}
