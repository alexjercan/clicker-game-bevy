//! UI plugin for the game.
use bevy::prelude::*;

use crate::core::*;

/// The UI component. This is added to spawn the UI.
#[derive(Component, Debug, Default)]
pub struct RootUI;

pub(super) struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelXPUIPlugin);
        app.add_plugins(SkillTreeUIPlugin);

        app.configure_sets(
            Update,
            LevelXPUIPluginSet.run_if(in_state(GameStates::Playing)),
        );
        app.configure_sets(
            Update,
            SkillTreeUIPluginSet.run_if(in_state(GameStates::Playing)),
        );

        app.add_systems(Update, setup_ui.run_if(in_state(GameStates::Playing)));
    }
}

fn setup_ui(mut commands: Commands, q_root: Query<Entity, With<RootUI>>, mut has_run: Local<bool>) {
    let Ok(root) = q_root.get_single() else {
        return;
    };

    if !*has_run {
        *has_run = true;
    } else {
        return;
    }

    commands
        .entity(root)
        .insert((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },))
        .with_children(|parent| {
            // Top bar for XP and Skill points alerts
            parent
                .spawn((
                    Name::new("TopBar"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Spawn the UI for the hex tiles.
                    parent.spawn((Name::new("HexTileUI"), LevelXPUI, Node::default()));

                    // Spawn the UI for the skill tree points.
                    parent.spawn((
                        Name::new("SkillTreePointsUI"),
                        SkillTreePointsUI,
                        Node::default(),
                    ));
                });
        });
}
