//! Fill Bar Plugin for the game

use bevy::prelude::*;

mod ui;

pub use ui::*;

/// The bar current value.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct FillBarValue(pub u32);

/// The bar maximum value. When the bar reaches this value an event is triggered.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct FillBarMaxValue(pub u32);

/// Event triggered when the bar reaches the maximum value.
/// The event contains the entity that reached the maximum value.
#[derive(Event, Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct FillBarValueReachedMax(pub Entity);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FillBarPluginSet;

#[derive(Debug, Default)]
pub struct FillBarPlugin;

impl Plugin for FillBarPlugin
{
    fn build(&self, app: &mut App) {
        app.add_plugins(FillBarUIPlugin);

        app.add_event::<FillBarValueReachedMax>();

        app.add_systems(Update, handle_fill_bar_value_reached_max.in_set(FillBarPluginSet));

        app.configure_sets(Update, FillBarUIPluginSet.in_set(FillBarPluginSet));
    }
}

fn handle_fill_bar_value_reached_max(
    q_fill_bar: Query<(Entity, &FillBarValue, &FillBarMaxValue)>,
    mut ev_fill_bar_value_reached_max: EventWriter<FillBarValueReachedMax>,
) {
    for (entity, value, max_value) in &q_fill_bar {
        if **value >= **max_value {
            ev_fill_bar_value_reached_max.send(FillBarValueReachedMax(entity));
        }
    }
}

