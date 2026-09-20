use bevy::prelude::*;
use clicker_tile::{GhostClicked, PlaceTile, TileClicked};

use crate::GameplaySystems;

const INITIAL_SKILL_POINTS: u32 = 1;
const INITIAL_XP_MAX: u32 = 10;
const XP_MAX_INCREMENT: u32 = 10;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct XpValue(pub u32);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct XpMax(pub u32);

impl Default for XpMax {
    fn default() -> Self {
        Self(INITIAL_XP_MAX)
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct SkillPoints(pub u32);

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelUp;

impl Default for SkillPoints {
    fn default() -> Self {
        Self(INITIAL_SKILL_POINTS)
    }
}

pub(crate) struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<XpValue>()
            .init_resource::<XpMax>()
            .init_resource::<SkillPoints>()
            .add_message::<LevelUp>()
            .add_systems(
                Update,
                (earn_xp, spend_skill_point).in_set(GameplaySystems::Progression),
            );
    }
}

fn earn_xp(
    mut events: MessageReader<TileClicked>,
    mut xp_value: ResMut<XpValue>,
    mut xp_max: ResMut<XpMax>,
    mut skill_points: ResMut<SkillPoints>,
    mut level_ups: MessageWriter<LevelUp>,
) {
    let earned = events.read().count() as u32;
    if earned == 0 {
        return;
    }
    xp_value.0 += earned;
    if apply_level_up(&mut xp_value.0, &mut xp_max.0, &mut skill_points.0) {
        level_ups.write(LevelUp);
    }
}

fn spend_skill_point(
    mut clicks: MessageReader<GhostClicked>,
    mut placements: MessageWriter<PlaceTile>,
    mut skill_points: ResMut<SkillPoints>,
) {
    for click in clicks.read() {
        if skill_points.0 == 0 {
            continue;
        }
        skill_points.0 -= 1;
        placements.write(PlaceTile(click.entity));
    }
}

fn apply_level_up(xp_value: &mut u32, xp_max: &mut u32, skill_points: &mut u32) -> bool {
    if *xp_value < *xp_max {
        return false;
    }
    *xp_value -= *xp_max;
    *skill_points += 1;
    *xp_max += XP_MAX_INCREMENT;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progression_defaults_are_explicit() {
        assert_eq!(XpValue::default().0, 0);
        assert_eq!(XpMax::default().0, INITIAL_XP_MAX);
        assert_eq!(SkillPoints::default().0, INITIAL_SKILL_POINTS);
    }

    #[test]
    fn xp_below_the_threshold_does_not_change_progression() {
        let mut xp = 9;
        let mut max = 10;
        let mut points = 0;
        assert!(!apply_level_up(&mut xp, &mut max, &mut points));
        assert_eq!((xp, max, points), (9, 10, 0));
    }

    #[test]
    fn xp_at_the_threshold_rolls_over_and_grants_one_skill_point() {
        let mut xp = 10;
        let mut max = 10;
        let mut points = 0;
        assert!(apply_level_up(&mut xp, &mut max, &mut points));
        assert_eq!((xp, max, points), (0, 20, 1));
    }
}
