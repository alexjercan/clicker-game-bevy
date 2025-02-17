#[cfg(feature = "debug")]
use debug::*;

use bevy::{asset::AssetMetaCheck, prelude::*};
use fill_bar::*;

struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Clicker".to_string(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        );

        app.add_plugins(FillBarPlugin);

        #[cfg(feature = "debug")]
        app.add_plugins(DebugPlugin);

        app.add_systems(Startup, setup);
        app.add_systems(Update, (handle_clicked_increase_xp, handle_xp_bar_reached_max));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera3D"),
        Camera3d::default(),
        Transform::from_xyz(-15.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let xp = commands
        .spawn((
            Name::new("TestingLevelXP"),
            FillBarValue(0),
            FillBarMaxValue(10),
        ))
        .id();

    commands
        .spawn((
            Name::new("UIRoot"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("XPBar"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                FillBarUI {
                    fill_color: Color::srgb(0.0, 0.5, 0.0),
                    ..default()
                },
                FillBarUITrack(xp),
            ));
        });
}

fn handle_clicked_increase_xp(
    mut q_xp: Query<&mut FillBarValue, With<FillBarMaxValue>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for mut xp in q_xp.iter_mut() {
            **xp += 1;
        }
    }
}

fn handle_xp_bar_reached_max(
    mut q_xp: Query<&mut FillBarValue, With<FillBarMaxValue>>,
    mut ev_xp_bar_reached_max: EventReader<FillBarValueReachedMax>,
) {
    for FillBarValueReachedMax(entity) in ev_xp_bar_reached_max.read() {
        if let Ok(mut xp) = q_xp.get_mut(*entity) {
            **xp = 0;
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DemoPlugin);
    app.run();
}
