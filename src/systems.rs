use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::ecs::system::ParamSet;
use bevy::render::mesh::{Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MaterialMesh2dBundle;
use crate::components::{GameplayCamera, Obstacle, Player, Floor, NonLethal, SelectedLevel, JumpBuffer, FinishLine};
use crate::levels::load_level;
use crate::states::GameState;
use std::time::Duration;

pub mod gameplay {
    use super::*;

    pub fn setup_gameplay(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        selected_level: Res<SelectedLevel>,
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

                let mut entity = commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(mesh).into(),
                        material: materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2))),
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
                    entity.insert(FinishLine);
                    entity.insert(ActiveEvents::COLLISION_EVENTS);
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
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(JumpBuffer {
                    timer: Timer::from_seconds(0.1, TimerMode::Once),
                });
        }

    }

    pub fn collision_event_system(
        mut collision_events: EventReader<CollisionEvent>,
        mut next_state: ResMut<NextState<GameState>>,
        obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
        non_lethal_query: Query<Entity, With<NonLethal>>,
        finish_query: Query<Entity, With<FinishLine>>, // Added for finish line detection
        player_query: Query<&Transform, With<Player>>,
    ) {
        for event in collision_events.read() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    if let Ok(player_transform) = player_query.get_single() {
                        // Check if collision involves an obstacle
                        let (obstacle_entity, obstacle_transform) = if let Ok((entity, transform)) = obstacle_query.get(*e1) {
                            (entity, transform)
                        } else if let Ok((entity, transform)) = obstacle_query.get(*e2) {
                            (entity, transform)
                        } else {
                            continue; // Skip if no obstacle is involved
                        };

                        // Check for finish line collision
                        if finish_query.get(obstacle_entity).is_ok() {
                            info!("Player reached the finish line!");
                            next_state.set(GameState::VictoryScreen); // Trigger victory screen
                            continue; // Skip further collision handling
                        }

                        // Check if the obstacle is non-lethal
                        let is_non_lethal = non_lethal_query.get(obstacle_entity).is_ok();
                        if is_non_lethal && is_top_collision(player_transform, obstacle_transform) {
                            // Allow jumping on top of non-lethal obstacles
                            continue;
                        } else {
                            // Handle lethal collision
                            handle_collision(&mut next_state);
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


    pub fn handle_collision(next_state: &mut ResMut<NextState<GameState>>) {
        info!("Player collided with an obstacle. Returning to Title Screen.");
        next_state.set(GameState::GameOver);
    }

    pub fn is_top_collision(player_transform: &Transform, obstacle_transform: &Transform) -> bool {
        let player_bottom = player_transform.translation.y - 15.0;
        let obstacle_top = obstacle_transform.translation.y + 12.5;
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
        time: Res<Time>,
        mut query: Query<(&mut Velocity, &mut JumpBuffer, &Transform), With<Player>>,
    ) {
        // Implement above with jump buffer (iter 2)
        for (mut velocity, mut jump_buffer, transform) in query.iter_mut() {
            velocity.linvel.x = 200.0;

            // reduce jump buffer time
            jump_buffer.timer.tick(time.delta());

            // store jump input in buffer
            if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) {
                jump_buffer.timer.reset();
            }

            // if player is on the ground and was recently pressed, apply force to jump
            if transform.translation.y <= 0.0 && jump_buffer.timer.elapsed_secs() < 0.1 {
                if velocity.linvel.y.abs() < 0.001 {
                    velocity.linvel.y = 300.0;
                    debug!("Buffered Jump Activated: Player Vertical velocity set to 300");
                    jump_buffer.timer.set_elapsed(Duration::from_secs_f32(0.1));
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