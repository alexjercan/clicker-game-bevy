use bevy::prelude::*;

use crate::{interaction::ClickedSelectedEvent, world::HexTile, GameplaySystems};

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct XpValue(pub u32);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct XpMax(pub u32);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct SkillPoints(pub u32);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ProgressionSystems {
    Earn,
    Level,
}

pub(crate) struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<XpValue>()
            .insert_resource(SkillPoints(1))
            .insert_resource(XpMax(10))
            .configure_sets(
                Update,
                (ProgressionSystems::Earn, ProgressionSystems::Level)
                    .chain()
                    .in_set(GameplaySystems::Progression),
            )
            .add_systems(
                Update,
                clicked_tile_increase_xp.in_set(ProgressionSystems::Earn),
            )
            .add_systems(
                Update,
                update_xp_value
                    .run_if(resource_changed::<XpValue>)
                    .in_set(ProgressionSystems::Level),
            );
    }
}

fn clicked_tile_increase_xp(
    mut events: MessageReader<ClickedSelectedEvent<HexTile>>,
    mut xp_value: ResMut<XpValue>,
) {
    for _ in events.read() {
        **xp_value += 1;
    }
}

fn update_xp_value(
    mut xp_value: ResMut<XpValue>,
    mut xp_max: ResMut<XpMax>,
    mut skill_points: ResMut<SkillPoints>,
) {
    apply_level_up(&mut xp_value.0, &mut xp_max.0, &mut skill_points.0);
}

fn apply_level_up(xp_value: &mut u32, xp_max: &mut u32, skill_points: &mut u32) {
    if *xp_value >= *xp_max {
        *xp_value -= *xp_max;
        *skill_points += 1;
        *xp_max += 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xp_below_the_threshold_does_not_change_progression() {
        let mut xp = 9;
        let mut max = 10;
        let mut points = 0;

        apply_level_up(&mut xp, &mut max, &mut points);

        assert_eq!((xp, max, points), (9, 10, 0));
    }

    #[test]
    fn xp_at_the_threshold_rolls_over_and_grants_one_skill_point() {
        let mut xp = 10;
        let mut max = 10;
        let mut points = 0;

        apply_level_up(&mut xp, &mut max, &mut points);

        assert_eq!((xp, max, points), (0, 20, 1));
    }
}
