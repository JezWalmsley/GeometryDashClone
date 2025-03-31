use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::ecs::system::ParamSet;
use bevy::render::mesh::{Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::audio::*;
use crate::components::{GameplayCamera, Obstacle, Player, Floor, NonLethal, SelectedLevel, FinishLine, LevelProgress, DeathSound, VictorySound, GameAudio, ProgressHistory, ProgressEntry};
use crate::levels::load_level;
use crate::states::GameState;
use chrono::Local;

pub mod gameplay {
    use crate::components::{DeathSound, ProgressText, VictorySound};
    use super::*;

    pub fn setup_gameplay(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        selected_level: Res<SelectedLevel>,
        asset_server: Res<AssetServer>,
    ) {
        info!("Setting up gameplay for level {}", selected_level.level_id);

        // Spawn the gameplay camera
        commands.spawn((
            Camera2dBundle::default(),
            GameplayCamera,
        ));
        debug!("Gameplay camera spawned.");

        // Load and set up the selected level
        if let Some(level) = load_level(selected_level.level_id) {
            info!("Level {} loaded successfully.", level.level_id);

            // Spawn obstacles
            for obstacle_data in level.obstacles.iter() {
                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default()
                );

                let positions = obstacle_data
                    .vertices
                    .iter()
                    .map(|&[x, y]| [x, y, 0.0])
                    .collect::<Vec<_>>();
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

                let indices = vec![0, 1, 2, 2, 3, 0];
                mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

                let color = obstacle_data.color.unwrap_or([0.8, 0.2, 0.2]);
                let material = materials.add(ColorMaterial::from(Color::srgb(color[0], color[1], color[2])));

                let mut entity = commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(mesh).into(),
                        material,
                        transform: Transform::from_translation(Vec3::new(
                            obstacle_data.position.x,
                            obstacle_data.position.y,
                            0.0,
                        )),
                        ..default()
                    },
                    Obstacle,
                    RigidBody::Fixed,
                    Collider::polyline(
                        obstacle_data.vertices.iter().map(|&[x, y]| Vec2::new(x, y)).collect(),
                        None,
                    ),
                ));

                if obstacle_data.non_lethal.unwrap_or(false) {
                    entity.insert(NonLethal);
                }

                if obstacle_data.is_finish.unwrap_or(false) {
                    info!("Finish line obstacle found at position: {:?}", obstacle_data.position);
                    entity.insert(FinishLine);
                    entity.insert(ActiveEvents::COLLISION_EVENTS); // Ensure collision events are triggered
                }
            }

            // Spawn the player
            commands
                .spawn(RigidBody::Dynamic)
                .insert(GravityScale(75.0))
                .insert(
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.0, 0.0, 1.0),
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..Default::default()
                        },
                        ..default()
                    },
                )
                .insert(TransformBundle::from(Transform::from_xyz(-200.0, 6.0, 0.0)))
                .insert(Velocity {
                    linvel: Vec2::new(1.0, 2.0),
                    angvel: 0.0,
                })
                .insert(Player)
                .insert(Collider::cuboid(15.0, 15.0))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .insert(LockedAxes::ROTATION_LOCKED);
        }
        // Progess text
        commands.spawn((
            TextBundle::from_section(
                "Progress: 0%",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            ProgressText,
        ));

    }


    // Collision Event System
    pub fn collision_event_system(
        mut collision_events: EventReader<CollisionEvent>,
        mut next_state: ResMut<NextState<GameState>>,
        obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
        non_lethal_query: Query<Entity, With<NonLethal>>,
        finish_query: Query<Entity, With<FinishLine>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        for event in collision_events.read() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    if let Ok(player_transform) = player_query.get_single() {
                        let (obstacle_entity, obstacle_transform) = if let Ok((entity, transform)) = obstacle_query.get(*e1) {
                            (entity, transform)
                        } else if let Ok((entity, transform)) = obstacle_query.get(*e2) {
                            (entity, transform)
                        } else {
                            continue;
                        };

                        // Check if the collision is with the finish line
                        if finish_query.get(obstacle_entity).is_ok() {
                            next_state.set(GameState::VictoryScreen);
                        } else {
                            let is_non_lethal = non_lethal_query.get(obstacle_entity).is_ok();
                            let player_size = Vec2::new(30.0, 30.0); // Assuming player size is 30x30
                            let obstacle_size = Vec2::new(25.0, 25.0); // Assuming obstacle size is 25x25
                            if is_non_lethal && is_top_collision(player_transform, player_size, obstacle_transform, obstacle_size) {
                                // Allow jumping on top of non-lethal obstacles
                                continue;
                            } else {
                                // Handle lethal collision
                                next_state.set(GameState::GameOver);
                            }
                        }
                    } else {
                        error!("Failed to retrieve player entity. Collision handling skipped.");
                    }
                }
                CollisionEvent::Stopped(_, _, _) => {
                    debug!("Collision stopped.");
                }
            }
        }
    }

    fn handle_collision(next_state: &mut ResMut<NextState<GameState>>) {
        next_state.set(GameState::GameOver);
    }

    pub fn is_top_collision(player_transform: &Transform, player_size: Vec2, obstacle_transform: &Transform, obstacle_size: Vec2) -> bool {
        let player_bottom = player_transform.translation.y - player_size.y / 2.0;
        let obstacle_top = obstacle_transform.translation.y + obstacle_size.y / 2.0;
        player_bottom > obstacle_top
    }

    pub fn cleanup_gameplay(
        mut commands: Commands,
        entities: Query<Entity, (Without<Camera>, Without<Window>)>,
        camera_entities: Query<Entity, With<GameplayCamera>>,
    ) {
        debug!("Cleaning up gameplay entities...");
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
            debug!("Entity {:?} despawned.", entity);
        }
        for entity in camera_entities.iter() {
            commands.entity(entity).despawn();
            debug!("Gameplay camera {:?} despawned.", entity);
        }
    }

    pub fn player_movement_system(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<&mut Velocity, With<Player>>,
    ) {
        for mut velocity in &mut query {
            velocity.linvel.x = 200.0;
            debug!("Player horizontal: {}", velocity.linvel.x);
            // debug!("Abs: {}", velocity.linvel.y.abs());

            if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) {
                if velocity.linvel.y.abs() < 0.001 {
                    velocity.linvel.y = 300.0;
                    debug!("Player vertical velocity set to 300.0.");
                }
            }
        }
    }

    pub fn exit_level_system(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut next_state: ResMut<NextState<GameState>>
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            info!("Player exited the level. Returning to Level Selection.");
            next_state.set(GameState::LevelSelection);
        }
    }

    pub fn level_scrolling_system(
        mut param_set: ParamSet<(
            Query<&Transform, With<Player>>,
            Query<&mut Transform, With<Obstacle>>,
            Query<&mut Transform, With<GameplayCamera>>,
        )>,
    ) {
        // Access the player's transform
        if let Ok(player_transform) = param_set.p0().get_single() {
            let player_x = player_transform.translation.x;

            // Update the camera's position
            for mut camera_transform in param_set.p2().iter_mut() {
                let target_camera_x = player_x + 100.0; // Offset camera ahead of the player
                camera_transform.translation.x = target_camera_x;
            }
        }
    }

    pub fn continuous_floor_system(
        mut param_set: ParamSet<(
            Query<&mut Transform, With<Floor>>,
            Query<&Transform, With<Player>>,
        )>,
        time: Res<Time>,
    ) {
        // Access the player's position
        if let Ok(player_transform) = param_set.p1().get_single() {
            let player_x = player_transform.translation.x;

            // Access and modify the floor's position
            let mut max_x = f32::MIN;

            for mut floor_transform in param_set.p0().iter_mut() {
                // Move floor segment to the left
                floor_transform.translation.x -= 200.0 * time.delta_seconds();

                // Track the farthest right floor segment
                if floor_transform.translation.x > max_x {
                    max_x = floor_transform.translation.x;
                }

                // Recycle floor segments that are too far left
                if floor_transform.translation.x < player_x - 500.0 {
                    floor_transform.translation.x = max_x + 100.0; // Adjust based on floor width
                }
            }
        }
    }

    pub fn spawn_floor(mut commands: Commands) {
        // Define floor segment dimensions
        let floor_width = 10000.0;
        let floor_height = 10.0;

        // Number of segments to cover the screen initially
        let num_segments = 100;

        for i in 0..num_segments {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 1.0, 1.0),
                        custom_size: Some(Vec2::new(floor_width, floor_height)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(i as f32 * floor_width, -36.0, 0.0),
                    ..Default::default()
                },
                Floor,
                RigidBody::Fixed,
                Collider::cuboid(floor_width / 2.0, floor_height / 2.0),
            ));
        }
    }
}

pub fn progress_tracker_system(
    player_query: Query<&Transform, With<Player>>,
    finish_query: Query<&Transform, With<FinishLine>>,
    mut progress: ResMut<LevelProgress>,
) {
    if let (Ok(player_transform), Ok(finish_transform)) =
        (player_query.get_single(), finish_query.get_single())
    {
        let start_x: f32 = -200.0;
        let player_x: f32 = player_transform.translation.x;
        let finish_x: f32 = finish_transform.translation.x;
        let total_distance = (finish_x - start_x).max(1.0); // Prevent division by zero
        let distance_traveled: f32 = (player_x - start_x).max(0.0);

        let progress_percentage = (distance_traveled / total_distance) * 100.0;
        progress.current_percentage = progress_percentage.clamp(0.0, 100.0); // **Fix applied here**

        // Save progress to leaderboard
        let file_path = "assets/progress.json";
        let mut history = ProgressHistory::load(file_path).unwrap_or_else(|_| ProgressHistory::default());
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let new_entry = ProgressEntry {
            date,
            percentage: progress.current_percentage, // Ensure clamped value is saved
        };
        history.add_entry(new_entry, file_path);
    }
}

pub fn setup_audio_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let death_sound = asset_server.load("audio/death.ogg");
    let victory_sound = asset_server.load("audio/victory.ogg");

    // Store audio handles as a resource
    commands.insert_resource(GameAudio {
        death_sound,
        victory_sound,
    });
}

// Play Death Sound
pub fn play_death_sound(mut commands: Commands, query: Query<Entity, With<DeathSound>>) {
    for entity in &query {
        commands.entity(entity).insert(PlaybackSettings::ONCE);
    }
}

// Play Victory Sound
pub fn play_victory_sound(mut commands: Commands, query: Query<Entity, With<VictorySound>>) {
    for entity in &query {
        commands.entity(entity).insert(PlaybackSettings::ONCE);
    }
}

pub fn play_sound(commands: &mut Commands, sound: Handle<AudioSource>) {
    commands.spawn(AudioBundle {
        source: sound,
        settings: PlaybackSettings::ONCE,
    });
}

pub fn setup_leaderboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let file_path = "assets/progress.json";
    let history = ProgressHistory::load(file_path);

    let mut leaderboard_text = "Leaderboard:\n".to_string();

    if let Ok(history) = ProgressHistory::load(file_path) {
        for entry in history.entries.iter().take(5) {
            leaderboard_text.push_str(&format!("{}: {:.1}%\n", entry.date, entry.percentage));
        }
    } else {
        warn!("Failed to load progress history. No leaderboard data available.");
    }

    commands.spawn((
        TextBundle::from_section(
            leaderboard_text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ),
    ));
}

