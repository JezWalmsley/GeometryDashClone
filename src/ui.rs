// ui.rs

use bevy::app::AppExit;
use bevy::color::Color::Srgba;
use bevy::prelude::*;

use crate::components::{QuitButton, StartButton, UICamera};
use crate::states::GameState;

pub fn setup_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a camera for the UI and tag it
    commands.spawn((
        Camera2dBundle::default(),
        UICamera, // Tag the camera
    ));

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
            background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)), // Black color
            ..default()
        })
        .with_children(|parent| {
            // Game title text
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Geometry Dash",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::srgb(1.0, 1.0, 1.0), // White color
                        },
                    ),
                    style: Style {
                        margin: UiRect::bottom(Val::Px(50.0)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                },
            );

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
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)), // Gray color
                        ..default()
                    },
                    StartButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Start",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::srgb(0.0, 0.0, 0.0), // Black color
                            },
                        ),
                        ..default()
                    });
                });

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
                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)), // Gray color
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Quit",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::srgb(0.0, 0.0, 0.0), // Black color
                            },
                        ),
                        ..default()
                    });
                });
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&StartButton>,
            Option<&QuitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color, start_button, quit_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25)); // Dark gray
                if start_button.is_some() {
                    // Transition to the Playing state
                    next_state.set(GameState::Playing);
                    info!("Play button pressed: Transitioning to Playing state");
                } else if quit_button.is_some() {
                    // Exit the application
                    std::process::exit(0);
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

pub fn cleanup_title_screen(
    mut commands: Commands,
    ui_entities: Query<Entity, With<Node>>,
    camera_entities: Query<Entity, With<UICamera>>, // Query for the UICamera
) {
    // Despawn all UI nodes
    for entity in &ui_entities {
        commands.entity(entity).despawn_recursive();
    }
    // Despawn the UI camera
    for entity in &camera_entities {
        commands.entity(entity).despawn();
    }
}
