// ui.rs
use bevy::app::AppExit;
use bevy::prelude::*;

use crate::components::{BackButton, LevelButton, QuitButton, SelectedLevel, StartButton, UICamera, RestartButton, GameOverText, ReturnToMenuButton, NextLevelButton, LevelProgress, ProgressText, LeaderboardButton, ProgressHistory};
use crate::states::GameState;

pub fn setup_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up the title screen...");

    // Spawn a camera for the UI
    commands.spawn((
        Camera2dBundle::default(),
        UICamera,
    ));
    debug!("UI camera spawned.");

    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            ..default()
        })
        .with_children(|parent| {
            // Game title text
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Geometry Dash",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });
            debug!("Game title text added to title screen.");

            // Start button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    StartButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
            debug!("Start button added to title screen.");

            // Edit Level button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.01, 0.01, 0.01)),
                        ..default()
                    },
                    LevelButton { level_id: 0 }, // Special marker for editor
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Edit Level",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            // color: Color::BLACK,
                            // gray color
                            color: Color::srgb(0.217, 0.217, 0.217),
                        },
                    ));
                });
            debug!("Edit Level button added to title screen.");

            // Quit button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
            debug!("Quit button added to title screen.");
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&StartButton>,
            Option<&QuitButton>,
            Option<&LevelButton>,
            Option<&LeaderboardButton>,
            Option<&BackButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color, start_button, quit_button, level_button, leaderboard_button, back_button) in
        interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                debug!("Button pressed.");
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));

                if start_button.is_some() {
                    info!("Start button pressed. Transitioning to Level Selection.");
                    next_state.set(GameState::LevelSelection);
                } else if quit_button.is_some() {
                    info!("Quit button pressed. Exiting...");
                    app_exit_events.send(AppExit::Success);
                } else if let Some(level_button) = level_button {
                    info!("Level {} selected. Transitioning to Playing state.", level_button.level_id);
                    next_state.set(GameState::Playing);
                } else if leaderboard_button.is_some() {
                    info!("Leaderboard button pressed. Transitioning to LeaderboardScreen.");
                    next_state.set(GameState::LeaderboardScreen);
                } else if back_button.is_some() {
                    info!("Back button pressed. Returning to Title Screen.");
                    next_state.set(GameState::TitleScreen);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.75, 0.75, 0.75));
                debug!("Button hovered.");
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
        }
    }
}


pub fn setup_level_selection(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up level selection screen...");

    // Spawn a camera for the UI
    commands.spawn((
        Camera2dBundle::default(),
        UICamera,
    ));
    debug!("UI camera for level selection screen spawned.");

    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        })
        .with_children(|parent| {
            // Title text
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Select Level",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            });
            debug!("Level selection title text added.");

            // Level buttons
            let level_count = 5; // Number of levels
            for level_id in 1..=level_count {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(300.0),
                                height: Val::Px(65.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                            ..default()
                        },
                        LevelButton { level_id },
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            format!("Level {}", level_id),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        ));
                    });
                debug!("Level {} button added.", level_id);
            }
        });

    // Back button
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                ..default()
            },
            BackButton,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Back",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });
}

pub fn level_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&LevelButton>,
            Option<&BackButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for (interaction, mut color, level_button, back_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25)); // Dark gray
                if let Some(level_button) = level_button {
                    // Store selected level
                    selected_level.level_id = level_button.level_id;
                    next_state.set(GameState::Playing);
                } else if back_button.is_some() {
                    next_state.set(GameState::TitleScreen);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.75, 0.75, 0.75)); // Light gray
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5)); // Gray
            }
        }
    }
}

pub fn cleanup_level_selection(
    mut commands: Commands,
    ui_entities: Query<Entity, With<Node>>,
    camera_entities: Query<Entity, With<UICamera>>,
) {
    // Despawn all UI nodes
    for entity in ui_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Despawn the UI camera
    for entity in camera_entities.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn setup_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up the Game Over menu...");

    // Spawn a UI camera
    commands.spawn((Camera2dBundle::default(), UICamera));

    // Root node for the menu
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)), // Dark background
            ..default()
        })
        .with_children(|parent| {
            // Game Over title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });

            // Restart button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    RestartButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            // Return to Menu button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    ReturnToMenuButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Return to Menu",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}

pub fn game_over_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&RestartButton>, Option<&ReturnToMenuButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, restart_button, return_to_menu_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25)); // Dark gray when pressed
                if restart_button.is_some() {
                    info!("Restart button pressed. Restarting level...");
                    next_state.set(GameState::Playing);
                } else if return_to_menu_button.is_some() {
                    info!("Return to Menu button pressed. Returning to main menu...");
                    next_state.set(GameState::TitleScreen);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.75, 0.75, 0.75)); // Light gray when hovered
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5)); // Default gray
            }
        }
    }
}

pub fn cleanup_game_over_menu(
    mut commands: Commands,
    ui_entities: Query<Entity, With<Node>>,
    camera_entities: Query<Entity, With<UICamera>>,
) {
    for entity in ui_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in camera_entities.iter() {
        commands.entity(entity).despawn();
    }
}


pub fn cleanup_title_screen(
    mut commands: Commands,
    ui_entities: Query<Entity, With<Node>>,
    camera_entities: Query<Entity, With<UICamera>>,
) {
    // Despawn all UI nodes
    for entity in ui_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Despawn the UI camera
    for entity in camera_entities.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn setup_victory_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up the Victory Screen...");

    // Spawn a UI camera
    commands.spawn((Camera2dBundle::default(), UICamera));

    // Root node for the Victory Screen UI
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)), // Dark background
            ..default()
        })
        .with_children(|parent| {
            // Victory Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Level Complete!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });

            // Next Level Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    NextLevelButton, // Marker component
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Next Level",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            // Restart Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    RestartButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            // Leaderboard button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    LeaderboardButton, // New marker component
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Leaderboard",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });


            // Return to Menu Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    ReturnToMenuButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Return to Menu",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}

pub fn victory_screen_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&NextLevelButton>,
            Option<&RestartButton>,
            Option<&ReturnToMenuButton>,
            Option<&LeaderboardButton>, // New leaderboard button
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for (interaction, mut color, next_level_button, restart_button, return_to_menu_button, leaderboard_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25)); // Dark gray when pressed

                if next_level_button.is_some() {
                    info!("Next Level button pressed.");
                    selected_level.level_id += 1; // Move to the next level
                    next_state.set(GameState::Playing);
                } else if restart_button.is_some() {
                    info!("Restart button pressed. Restarting current level...");
                    next_state.set(GameState::Playing);
                } else if return_to_menu_button.is_some() {
                    info!("Return to Menu button pressed. Going back to the main menu...");
                    next_state.set(GameState::TitleScreen);
                } else if leaderboard_button.is_some() {
                    info!("Leaderboard button pressed. Showing leaderboard for level {}.", selected_level.level_id);
                    next_state.set(GameState::LeaderboardScreen); // New game state for leaderboard
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.75, 0.75, 0.75)); // Light gray when hovered
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5)); // Default gray
            }
        }
    }
}


pub fn cleanup_victory_screen(
    mut commands: Commands,
    ui_entities: Query<Entity, With<Node>>,
    camera_entities: Query<Entity, With<Camera>>,
) {
    // Despawn all UI nodes related to the Victory Screen
    for entity in ui_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn the UI camera for the Victory Screen
    for entity in camera_entities.iter() {
        commands.entity(entity).despawn();
    }

    info!("Victory Screen and UI Camera cleaned up successfully.");
}

pub fn update_progress_ui(
    progress: Res<LevelProgress>,
    mut query: Query<&mut Text, With<ProgressText>>, // Marker for progress text
) {
    if progress.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Progress: {:.1}%", progress.current_percentage);
        }
    }
}

pub fn setup_leaderboard_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up the leaderboard screen...");

    // Spawn a UI camera
    commands.spawn((Camera2dBundle::default(), UICamera));

    // Root UI node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Leaderboard",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            });

            // Leaderboard text
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Loading...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });

            // **Back Button**
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                        ..default()
                    },
                    BackButton, // This component ensures it works in button_system
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            debug!("Leaderboard back button added.");
        });
}


pub fn cleanup_leaderboard_screen(
    mut commands: Commands,
    ui_entities: Query<Entity, With<Node>>,
    camera_entities: Query<Entity, With<Camera>>,
) {
    // Despawn all UI nodes related to the Leaderboard Screen
    for entity in ui_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn the UI camera for the Leaderboard Screen
    for entity in camera_entities.iter() {
        commands.entity(entity).despawn();
    }

    info!("Leaderboard Screen and UI Camera cleaned up successfully.");
}


