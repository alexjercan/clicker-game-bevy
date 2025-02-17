//! Select Menu Plugin

use bevy::prelude::*;

mod ui;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectMenuPluginSet;

#[derive(Debug, Default)]
pub struct SelectMenuPlugin;

impl Plugin for SelectMenuPlugin
{
    fn build(&self, app: &mut App) {}
}
