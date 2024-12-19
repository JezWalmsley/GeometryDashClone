use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::ecs::system::ParamSet;

pub mod gameplay {
    use super::*;
    use crate::components::{GameplayCamera, Obstacle, Player, Floor};
    use crate::levels::{get_level};
    use crate::components::SelectedLevel;
    use crate::states::GameState;

    pub fn setup_gameplay(
        mut commands: Commands,
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
        if let Some(level) = get_level(selected_level.level_id) {
            info!("Level {} loaded successfully.", level.level_id);

            for obstacle_data in &level.obstacles {
                debug!(
                    "Spawning obstacle at position {:?} with size {:?}",
                    obstacle_data.position, obstacle_data.size
                );
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.8, 0.2, 0.2),
                            custom_size: Some(obstacle_data.size),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            obstacle_data.position.x,
                            obstacle_data.position.y,
                            0.0,
                        )),
                        ..default()
                    },
                    Obstacle,
                    RigidBody::Fixed,
                    Collider::cuboid(
                        obstacle_data.size.x / 2.0,
                        obstacle_data.size.y / 2.0,
                    ),
                ));
            }
        } else {
            error!("Failed to load level {}. Check if it exists in `get_level`.", selected_level.level_id);
        }

        // Spawn the player
        // commands.spawn((
        //     SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::rgb(0.0, 0.0, 1),
        //             custom_size: Some(Vec2::new(30.0, 30.0)),
        //             ..default()
        //         },
        //         transform: Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0)),
        //         ..default()
        //     },
        //     Player,
        //     RigidBody::Dynamic,
        //     Collider::cuboid(15.0, 15.0),
        //     Velocity::zero(),
        //     GravityScale(1.0),
        //     ActiveEvents::COLLISION_EVENTS,
        // ));
        // debug!("Player spawned at position (-200.0, 0.0).");
        commands
            .spawn(RigidBody::Dynamic)
            .insert(GravityScale(75.0))
            .insert(
                (SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.0, 0.0, 1.0),
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..Default::default()
                    },
                    // transform: Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0)),
                    ..default()
                },
                    //Player,
                    // Collider::cuboid(15.0, 15.0),
                    //Velocity::zero(),
                    //GravityScale(1.0),
                    // ActiveEvents::COLLISION_EVENTS,
                )
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

        // spawn floor
        // commands
        //     .spawn(RigidBody::Fixed)
        //     .insert(
        //         Collider::cuboid(2000.0, 3.0)
        //     )
        //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)))
        //     .insert(SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::srgb(0.0, 0.0, 0.0),
        //             custom_size: Some(Vec2::new(2000.0, 3.0)),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     });
    }

    pub fn collision_event_system(
        mut collision_events: EventReader<CollisionEvent>,
        mut next_state: ResMut<NextState<GameState>>,
        obstacle_query: Query<Entity, With<Obstacle>>,
        player_query: Query<Entity, With<Player>>,
    ) {
        // debug!("Processing collision events...");
        for event in collision_events.read() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    debug!("Collision started between entities {:?} and {:?}.", e1, e2);

                    if let Ok(player_entity) = player_query.get_single() {
                        if (*e1 == player_entity && obstacle_query.get(*e2).is_ok())
                            || (*e2 == player_entity && obstacle_query.get(*e1).is_ok())
                        {
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
        next_state.set(GameState::LevelSelection);
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
            //debug!("Player horizontal velocity set to 200.0.");
            debug!("Abs: {}", velocity.linvel.y.abs());

            if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) {
                if velocity.linvel.y.abs() < 0.001 {
                    velocity.linvel.y = 300.0;
                    debug!("Player vertical velocity set to 300.0.");
                }
            }
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

            // Move obstacles
            // for mut obstacle_transform in param_set.p1().iter_mut() {
            //     obstacle_transform.translation.x -= 200.0 * time.delta_seconds();
            //
            //     // Optionally, reset obstacles that move off-screen
            //     if obstacle_transform.translation.x < player_x - 500.0 {
            //         obstacle_transform.translation.x = player_x + 500.0;
            //     }
            // }
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
                        color: Color::srgb(1.0, 0.0, 0.0),
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
