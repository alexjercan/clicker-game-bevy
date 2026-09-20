use bevy::prelude::*;
use clicker_audio::{AudioSystems, SfxCue};
use clicker_gameplay::{
    LevelUp, TileClicked, TileDeselected, TileKind, TilePlaced, TileSelected, TileSystems,
};
use clicker_state::GameState;

pub(super) struct AudioGluePlugin;

impl Plugin for AudioGluePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, AudioSystems::Playback.after(TileSystems::Feedback))
            .add_systems(
                Update,
                (
                    request_click_sfx,
                    request_selection_sfx,
                    request_placement_sfx,
                    request_level_up_sfx,
                )
                    .in_set(TileSystems::Feedback)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn request_click_sfx(
    mut clicks: MessageReader<TileClicked>,
    kinds: Query<&TileKind>,
    mut cues: MessageWriter<SfxCue>,
) {
    for click in clicks.read() {
        if let Ok(kind) = kinds.get(click.entity) {
            cues.write(SfxCue::TileClick(*kind));
        }
    }
}

fn request_selection_sfx(
    mut selected: MessageReader<TileSelected>,
    mut deselected: MessageReader<TileDeselected>,
    mut cues: MessageWriter<SfxCue>,
) {
    let selected_changed = selected.read().count() > 0;
    let deselected_changed = deselected.read().count() > 0;
    if selected_changed || deselected_changed {
        cues.write(SfxCue::Selection);
    }
}

fn request_placement_sfx(mut placed: MessageReader<TilePlaced>, mut cues: MessageWriter<SfxCue>) {
    for _ in placed.read() {
        cues.write(SfxCue::Placement);
    }
}

fn request_level_up_sfx(mut level_ups: MessageReader<LevelUp>, mut cues: MessageWriter<SfxCue>) {
    for _ in level_ups.read() {
        cues.write(SfxCue::LevelUp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clicker_gameplay::TileCoord;

    #[derive(Resource, Default)]
    struct CollectedCues(Vec<SfxCue>);

    fn collect_cues(mut cues: MessageReader<SfxCue>, mut collected: ResMut<CollectedCues>) {
        collected.0.extend(cues.read());
    }

    #[test]
    fn tile_click_requests_the_cue_for_its_kind() {
        let mut app = App::new();
        app.add_message::<TileClicked>()
            .add_message::<SfxCue>()
            .init_resource::<CollectedCues>()
            .add_systems(Update, (request_click_sfx, collect_cues).chain());
        let entity = app.world_mut().spawn(TileKind::Stone).id();
        app.world_mut().write_message(TileClicked {
            entity,
            coord: TileCoord::ZERO,
        });

        app.update();

        assert_eq!(
            app.world().resource::<CollectedCues>().0,
            vec![SfxCue::TileClick(TileKind::Stone)]
        );
    }

    #[test]
    fn placement_and_level_up_request_distinct_cues() {
        let mut app = App::new();
        app.add_message::<TilePlaced>()
            .add_message::<LevelUp>()
            .add_message::<SfxCue>()
            .init_resource::<CollectedCues>()
            .add_systems(
                Update,
                (request_placement_sfx, request_level_up_sfx, collect_cues).chain(),
            );
        app.world_mut().write_message(TilePlaced {
            coord: TileCoord::ZERO,
            kind: TileKind::Tree,
        });
        app.world_mut().write_message(LevelUp);

        app.update();

        assert_eq!(
            app.world().resource::<CollectedCues>().0,
            vec![SfxCue::Placement, SfxCue::LevelUp]
        );
    }

    #[test]
    fn simultaneous_deselect_and_select_request_one_selection_cue() {
        let mut app = App::new();
        app.add_message::<TileSelected>()
            .add_message::<TileDeselected>()
            .add_message::<SfxCue>()
            .init_resource::<CollectedCues>()
            .add_systems(Update, (request_selection_sfx, collect_cues).chain());
        let old_entity = app.world_mut().spawn_empty().id();
        let new_entity = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(TileDeselected {
            entity: old_entity,
            coord: TileCoord::ZERO,
        });
        app.world_mut().write_message(TileSelected {
            entity: new_entity,
            coord: TileCoord::new(1, 0),
        });

        app.update();

        assert_eq!(
            app.world().resource::<CollectedCues>().0,
            vec![SfxCue::Selection]
        );
    }
}
