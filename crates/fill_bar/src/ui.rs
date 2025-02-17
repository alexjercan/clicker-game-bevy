//! UI Plugin for the Fill Bar of the plugin.

use bevy::prelude::*;

use super::{FillBarMaxValue, FillBarValue};

/// Component used to indicate where the Fill Bar UI should be placed.
#[derive(Component, Debug)]
pub struct FillBarUI {
    pub fill_bar_width: f32,
    pub fill_bar_height: f32,
    pub background_color: Color,
    pub fill_color: Color,
}

/// This component will store the entity that this UI element has to track.
///
/// This should be replaced in the future by Bevy Relationships.
#[derive(Component, Debug, Deref, DerefMut)]
pub struct FillBarUITrack(pub Entity);

impl Default for FillBarUI {
    fn default() -> Self {
        Self {
            fill_bar_width: 200.0,
            fill_bar_height: 24.0,
            background_color: Color::WHITE,
            fill_color: Color::BLACK,
        }
    }
}

/// Marker component used to indicate that the fill bar was spawned.
#[derive(Component, Debug, Default)]
struct FillBarUISpawned;

/// Component used to indicate the fill of the fill bar.
///
/// This component will store a reference to the parent UI Root.
/// Again, this is used because we don't have real relationsihps in bevy.
#[derive(Component, Debug, Deref, DerefMut)]
struct FillBarUIFill(Entity);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FillBarUIPluginSet;

#[derive(Debug)]
pub struct FillBarUIPlugin;

impl Plugin for FillBarUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (setup_bar_ui, update_fill_bar_ui).in_set(FillBarUIPluginSet),
        );
    }
}

fn setup_bar_ui(
    mut commands: Commands,
    q_root: Query<(Entity, &FillBarUI), Without<FillBarUISpawned>>,
) {
    for (root, FillBarUI { fill_bar_width, fill_bar_height, background_color, fill_color }) in &q_root {
        commands
            .entity(root)
            .insert(FillBarUISpawned)
            .with_children(|parent| {
                parent
                    .spawn((
                        Name::new("FillBarUI_BarBackground"),
                        Node {
                            width: Val::Px(*fill_bar_width),
                            height: Val::Px(*fill_bar_height),
                            ..default()
                        },
                        BackgroundColor(*background_color),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("FillBarUI_BarFill"),
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(*fill_color),
                            FillBarUIFill(root),
                        ));
                    });
            });
    }
}

fn update_fill_bar_ui(
    q_value: Query<(&FillBarValue, &FillBarMaxValue), Changed<FillBarValue>>,
    q_root: Query<&FillBarUITrack, With<FillBarUI>>,
    mut q_fill: Query<(&mut Node, &FillBarUIFill)>,
) {
    for (mut node, FillBarUIFill(root)) in &mut q_fill {
        if let Ok(FillBarUITrack(track)) = q_root.get(*root) {
            if let Ok((value, max_value)) = q_value.get(*track) {
                let frac = **value as f32 / **max_value as f32;
                node.width = Val::Percent((frac * 100.0).min(100.0));
            }
        }
    }
}
