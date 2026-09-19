use std::marker::PhantomData;

use bevy::prelude::*;
use clicker_animation::{deselect_tile, select_tile};

use crate::{
    world::{HexCoord, HexGhost, HexMap, HexTile},
    GameplaySystems,
};

#[derive(Component, Debug, Clone)]
struct Selected;

#[derive(Message)]
pub(crate) struct SelectedEvent<T: Component> {
    entity: Entity,
    marker: PhantomData<T>,
}

impl<T: Component> SelectedEvent<T> {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            marker: PhantomData,
        }
    }

    pub(crate) fn entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Message)]
pub(crate) struct DeselectedEvent<T: Component> {
    entity: Entity,
    marker: PhantomData<T>,
}

impl<T: Component> DeselectedEvent<T> {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            marker: PhantomData,
        }
    }

    pub(crate) fn entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Message)]
pub(crate) struct ClickedSelectedEvent<T: Component> {
    entity: Entity,
    marker: PhantomData<T>,
}

impl<T: Component> ClickedSelectedEvent<T> {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            marker: PhantomData,
        }
    }

    pub(crate) fn entity(&self) -> Entity {
        self.entity
    }
}

pub(crate) struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ClickedSelectedEvent<HexGhost>>()
            .add_message::<ClickedSelectedEvent<HexTile>>()
            .add_message::<SelectedEvent<HexTile>>()
            .add_message::<DeselectedEvent<HexTile>>()
            .add_systems(Update, update_selected.in_set(GameplaySystems::Selection))
            .add_systems(
                Update,
                (selected_tile_tween, deselect_tile_tween)
                    .in_set(GameplaySystems::SelectionFeedback),
            )
            .add_systems(
                Update,
                (
                    update_click_selected_hex::<HexTile>,
                    update_click_selected_hex::<HexGhost>,
                )
                    .in_set(GameplaySystems::Click),
            );
    }
}

fn update_selected(
    mut commands: Commands,
    hexmap: Res<HexMap>,
    windows: Query<&Window>,
    q_hex: Query<(Entity, &HexCoord), Without<Selected>>,
    q_selected: Query<(Entity, &HexCoord), With<Selected>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut events: SelectionEvents,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };

    let cursor_coord = hexmap.world_pos_to_hex(ray.get_point(distance).xz());

    if let Ok((entity, hex_coord)) = q_selected.single() {
        if **hex_coord != cursor_coord {
            commands.entity(entity).remove::<Selected>();
            events.deselected.write(DeselectedEvent::new(entity));
        } else {
            return;
        }
    }

    if let Some((entity, _)) = q_hex
        .iter()
        .find(|(_, HexCoord(hex_coord))| *hex_coord == cursor_coord)
    {
        commands.entity(entity).insert(Selected);
        events.selected.write(SelectedEvent::new(entity));
    }
}

#[derive(bevy::ecs::system::SystemParam)]
struct SelectionEvents<'w> {
    selected: MessageWriter<'w, SelectedEvent<HexTile>>,
    deselected: MessageWriter<'w, DeselectedEvent<HexTile>>,
}

fn selected_tile_tween(mut commands: Commands, mut events: MessageReader<SelectedEvent<HexTile>>) {
    for event in events.read() {
        commands.entity(event.entity()).insert(select_tile());
    }
}

fn deselect_tile_tween(
    mut commands: Commands,
    mut events: MessageReader<DeselectedEvent<HexTile>>,
) {
    for event in events.read() {
        commands.entity(event.entity()).insert(deselect_tile());
    }
}

fn update_click_selected_hex<T: Component>(
    buttons: Res<ButtonInput<MouseButton>>,
    selected: Query<Entity, (With<Selected>, With<T>)>,
    mut events: MessageWriter<ClickedSelectedEvent<T>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(entity) = selected.single() {
            events.write(ClickedSelectedEvent::new(entity));
        }
    }
}
