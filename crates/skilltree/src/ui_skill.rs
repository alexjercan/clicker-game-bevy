//! UI Plugin for the Skill Tree XP

use bevy::prelude::*;

use super::{AvailableSkills, SkillTreePoints};

/// The assets used for showing UI
#[derive(Resource, Clone, Debug, Default)]
pub struct SkillTreeUIAssets {
    pub skill_tree_points: Handle<Image>,
}

#[derive(Debug, Clone, Event)]
pub struct SkillTreeChooseButtonPressed<T>
where
    T: Component + Clone + Copy + Default,
{
    pub skill: T,
}

/// The root of the skill tree UI. This component is used to indicate where the skill tree UI
/// should be placed.
#[derive(Component, Debug, Default)]
pub struct SkillTreeUIRoot;

/// Component used to indicate the description of the skill tree.
#[derive(Component, Debug, Default)]
pub struct SkillTreeUILevelDescription;

/// Component used to indicate a skill in the skill tree.
#[derive(Component, Debug, Default)]
struct SkillTreeUIGrid;

/// Component used to indicate which skill is selected in the given option.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct SkillTreeUISkillSelected(pub u32);

/// Component used to find the choose button in the skill tree.
#[derive(Component, Debug, Default)]
pub struct SkillTreeUIChooseButton;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillTreeUIPluginSet;

#[derive(Debug, Default)]
pub struct SkillTreeUIPlugin<T>
where
    T: Component + Clone + Copy + Default,
{
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Debug, Component, Default)]
pub struct SelectedOption<T>
where
    T: Component + Clone + Copy + Default,
{
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Plugin for SkillTreeUIPlugin<T>
where
    T: Component + Clone + Copy + Default,
{
    fn build(&self, app: &mut App) {
        // TODO: Can we have a better way to add assets?
        //
        // One option that I can think of is to have the path to the asset in the plugin and then
        // we load them via a resource. Since they are also being loaded by bevy_asset_loader, we
        // can just use them directly. It is kind of scuffed, but it can work. For now I will leave
        // it as it is.
        app.init_resource::<SkillTreeUIAssets>();

        app.add_event::<SkillTreeChooseButtonPressed<T>>();

        app.add_systems(
            Update,
            // Systems used to create the UI layout elements
            setup_skill_ui::<T>.in_set(SkillTreeUIPluginSet),
        );

        app.add_systems(
            Update,
            (
                // Update the text that shows you how many skill points you have available
                update_skill_tree_description_points_ui,
                // Update the skill tree options
                update_skill_tree_options::<T>.run_if(resource_changed::<AvailableSkills<T>>),
                handle_button_interact::<T>,
                update_selected_skill_button::<T>,
                skill_tree_menu_action::<T>,
            )
                .in_set(SkillTreeUIPluginSet),
        );
    }
}

fn setup_skill_ui<T>(
    mut commands: Commands,
    q_root: Query<Entity, With<SkillTreeUIRoot>>,
    ui_assets: Res<SkillTreeUIAssets>,
    skills: Res<AvailableSkills<T>>,
    mut has_run: Local<bool>,
) where
    T: Component + Clone + Copy + Default,
{
    let Ok(root) = q_root.get_single() else {
        return;
    };

    if !*has_run {
        *has_run = true;
    } else {
        return;
    }

    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                Name::new("SkillTreeUI_Root"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            ))
            .with_children(|parent| {
                // Text with "Level Up"
                parent
                    .spawn((
                        Name::new("SkillTreeUI_LevelUp"),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(15.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    ))
                    .with_child((
                        Name::new("SkillTreeUI_LevelUpText"),
                        Text::new("Level Up"),
                        TextFont {
                            font_size: 35.0,
                            ..default()
                        },
                        Label,
                    ));

                // Description with the level count
                parent
                    .spawn((
                        Name::new("SkillTreeUI_LevelDescription"),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(25.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("SkillTreeUI_Image"),
                            Node {
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ImageNode::new(ui_assets.skill_tree_points.clone()),
                        ));
                        parent
                            .spawn((
                                Name::new("SkillTreeUI_LevelText"),
                                Node {
                                    height: Val::Percent(100.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::FlexStart,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Name::new("SkillTreeUI_LevelText_Text"),
                                    Text::new(format!("You have {} skill points available.", 0)),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    Label,
                                    SkillTreeUILevelDescription,
                                ));
                            });
                    });

                // Skill Tree Grid 2x2 with 4 available skills to improve
                parent
                    .spawn((
                        Name::new("SkillTreeUI_Grid"),
                        Node {
                            display: Display::Grid,
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                            row_gap: Val::Px(12.0),
                            column_gap: Val::Px(12.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                        SkillTreeUIGrid,
                    ))
                    .with_children(|parent| {
                        spawn_skills(parent, &skills);
                    });

                // Button to choose the skill
                parent
                    .spawn((
                        Name::new("SkillTreeUI_Button"),
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(NORMAL_BUTTON),
                        SkillTreeUIChooseButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("SkillTreeUI_ButtonText"),
                            Text::new("OK"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            Label,
                        ));
                    });
            });
    });
}

fn update_skill_tree_description_points_ui(
    q_points: Query<&SkillTreePoints>,
    mut q_description: Query<&mut Text, With<SkillTreeUILevelDescription>>,
) {
    let Ok(points) = q_points.get_single() else {
        return;
    };

    for mut text in &mut q_description {
        **text = format!("You have {} skill points available.", **points);
    }
}

fn update_skill_tree_options<T>(
    mut commands: Commands,
    skills: Res<AvailableSkills<T>>,
    q_grid: Query<Entity, With<SkillTreeUIGrid>>,
) where
    T: Component + Clone + Copy + Default,
{
    let Ok(grid) = q_grid.get_single() else {
        return;
    };

    commands
        .entity(grid)
        .despawn_descendants()
        .with_children(|parent| {
            spawn_skills(parent, &skills);
        });
}

fn spawn_skills<T>(parent: &mut ChildBuilder, skills: &AvailableSkills<T>)
where
    T: Component + Clone + Copy + Default,
{
    for (_, skill) in skills.iter().take(4).enumerate() {
        parent
            .spawn((
                Name::new("SkillTreeUI_Skill"),
                Node {
                    display: Display::Grid,
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK.into()),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Name::new("SkillTreeUI_SkillButton"),
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(NORMAL_BUTTON),
                        skill.component,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("SkillTreeUI_SkillImage"),
                            Node {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                ..default()
                            },
                            ImageNode::new(skill.icon.clone()),
                        ));
                        parent.spawn((
                            Name::new("SkillTreeUI_SkillText"),
                            Text::new(skill.name.clone()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            Label,
                        ));
                    });
            });
    }
}

// This system handles changing all buttons color based on mouse interaction
fn handle_button_interact<T>(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SelectedOption<T>>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) where
    T: Component + Clone + Copy + Default,
{
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn update_selected_skill_button<T>(
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>, With<T>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption<T>>>,
    mut commands: Commands,
) where
    T: Component + Clone + Copy + Default,
{
    for (interaction, entity) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok((previous_button, mut previous_button_color)) =
                selected_query.get_single_mut()
            {
                *previous_button_color = NORMAL_BUTTON.into();
                commands
                    .entity(previous_button)
                    .remove::<SelectedOption<T>>();
            }

            commands
                .entity(entity)
                .insert(SelectedOption::<T>::default());
        }
    }
}

fn skill_tree_menu_action<T>(
    interaction_query: Query<
        (&Interaction, &SkillTreeUIChooseButton),
        (Changed<Interaction>, With<Button>),
    >,
    q_selected: Query<&T, With<SelectedOption<T>>>,
    mut events: EventWriter<SkillTreeChooseButtonPressed<T>>,
) where
    T: Component + Clone + Copy + Default,
{
    let Ok(selected) = q_selected.get_single() else {
        return;
    };

    for (interaction, _menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            events.send(SkillTreeChooseButtonPressed { skill: *selected });
        }
    }
}
