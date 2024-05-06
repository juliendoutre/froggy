use crate::{despawn_screen, GameState};
use bevy::{app::AppExit, prelude::*};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup)
        .add_systems(
            Update,
            (button_looking, button_action).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
}

// All actions that can be triggered from a button click.
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Resource)]
struct ButtonHoverSound(Handle<AudioSource>);

#[derive(Resource)]
struct ButtonClickSound(Handle<AudioSource>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let borders = asset_server.load("./menu-border.png");
    let borders_slicer = TextureSlicer {
        border: BorderRect::square(22.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    commands.insert_resource(ButtonHoverSound(asset_server.load("./button-hover.ogg")));
    commands.insert_resource(ButtonClickSound(asset_server.load("./button-click.ogg")));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Froggy's adventures",
                    TextStyle {
                        font_size: 60.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect {
                        top: Val::Px(40.0),
                        bottom: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    z_index: ZIndex::Global(-1),
                    style: Style {
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(100.0),
                        margin: UiRect {
                            top: Val::Px(40.0),
                            bottom: Val::Px(40.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: borders.clone().into(),
                            z_index: ZIndex::Global(-1),
                            style: Style {
                                height: Val::Vh(60.0),
                                width: Val::Vw(40.0),
                                ..default()
                            },
                            ..default()
                        },
                        ImageScaleMode::Sliced(borders_slicer.clone()),
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                height: Val::Percent(80.0),
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: Color::NONE.into(),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(30.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    MenuButtonAction::Play,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Play",
                                        TextStyle {
                                            font_size: 40.0,
                                            color: TEXT_COLOR,
                                            ..default()
                                        },
                                    ));
                                });

                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: Color::NONE.into(),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(30.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    MenuButtonAction::Quit,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Quit",
                                        TextStyle {
                                            font_size: 40.0,
                                            color: TEXT_COLOR,
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            format!("version {}", env!("CARGO_PKG_VERSION")),
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            margin: UiRect {
                                top: Val::Px(20.0),
                                right: Val::Px(20.0),
                                ..default()
                            },
                            right: Val::Px(0.0),
                            ..default()
                        }),
                    );
                });
        });
}

fn button_looking(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    button_hover_sound: Res<ButtonHoverSound>,
    button_click_sound: Res<ButtonClickSound>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                commands.spawn(AudioBundle {
                    source: button_hover_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            Interaction::None => {}
            Interaction::Pressed => {
                commands.spawn(AudioBundle {
                    source: button_click_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}

fn button_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                }
            }
        }
    }
}
