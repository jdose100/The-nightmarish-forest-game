//! implementation of player systems
use std::f32::consts::{FRAC_PI_2, PI};

// import crates
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_rapier3d::{na::wrap, prelude::*};
use autodefault::autodefault;

// import data from this crate
use crate::components::SphereOfTear;
use super::structures::*;

/// Create and setup player
#[autodefault]
pub fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    have_player: Query<Option<&PlayerComponent>>,
) {
    // if the player already exists, then exit
    if let Ok(_) = have_player.get_single() {
        return;
    }

    // setup player
    commands.spawn((
        #[cfg(debug_assertions)] Name::new("Player"),
        Transform::from_xyz(0.0, 6.0, 0.0), // general data
        PlayerControllerData::default(),
        InheritedVisibility::HIDDEN,
        PlayerComponent::default(),

        // Mesh3d(meshes.add(Cylinder::new(PLAYER_RADIUS, PLAYER_HEIGHT))),
        // MeshMaterial3d(materials.add(StandardMaterial::default())),

        RigidBody::KinematicVelocityBased, // physics data
        Collider::cylinder(PLAYER_HEIGHT, PLAYER_RADIUS),
    )).with_children(|parent| {
            parent.spawn(( // add player's camera
                Projection::from(PerspectiveProjection { fov: 90.0_f32.to_radians() }),
                Transform::from_translation(CAMERA_WALK_TRANSLATION),
                PlayerCameraPivot {},
                Camera3d::default(),

                // Mesh3d(meshes.add(Cuboid::default())),
                // MeshMaterial3d(materials.add(StandardMaterial::default())),
            ));
        });
}

/// get player's input from keyboard
pub fn update_input(
    input_enabled: Res<PlayerInputEnabled>,
    player_query: Query<&PlayerComponent>,
    keys: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<PlayersInput>
) {
    if !input_enabled.0 { // if input disabled, exit
        return;
    } else if let Err(_) = player_query.get_single() {
        return;
    }

    // get player data
    let player_data = player_query.single();

    // get direction input
    input.backward = keys.pressed(KeyCode::KeyS);
    input.forward = keys.pressed(KeyCode::KeyW);
    input.right = keys.pressed(KeyCode::KeyD);
    input.left = keys.pressed(KeyCode::KeyA);

    // get action's input
    input.crouch = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    // if stamina > 5.0, player can't run
    let run = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    input.run = run && !player_data.stop_run;

    // can't jump while crouch
    input.jump = keys.pressed(KeyCode::Space) && !input.crouch;
}

/// set visible cursor
pub fn update_cursor_visible(
    mut input_enabled: ResMut<PlayerInputEnabled>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut window in window_query.iter_mut() {
        if buttons.just_pressed(MouseButton::Right) {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
            input_enabled.0 = true;
        }

        if keys.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            input_enabled.0 = false;
        }
    }
}

/// set cursor position = window_size / 2.0
pub fn update_cursor_position(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    input_enabled: ResMut<PlayerInputEnabled>,
) {
    if !input_enabled.0 {
        return;
    }

    for mut window in window_query.iter_mut() {
        let position = (window.size() / 2.0) + 2.0;
        window.set_cursor_position(Some(position));
    }
}


/// update player's stamina
pub fn update_stamina(
    mut player_query: Query<(&mut PlayerComponent, &PlayerControllerData)>,
    input: Res<PlayersInput>,
    input_enabled: Res<PlayerInputEnabled>,
) {
    if !input_enabled.0 {
        return;
    }

    let Ok((
        mut player_data, player_controller
    )) = player_query.get_single_mut() else { return; };

    // if player not on ground not need
    // update stamina
    if !player_controller.grounded {
        return;
    }

    if input.run && (input.backward || input.forward || input.right || input.left) && !player_controller.crouched {
        // if stamina > 0.0 and player can run
        // decrase stamina rate
        if player_data.stamina >= 0.0 && !player_data.stop_run {
            player_data.stamina -= STAMINA_DECRASE_RATE;
        }

        // if stamina < 0.0 and player "stop_run" flag not set:
        // set stop_run = true (player can't run)
        if player_data.stamina <= 0.0 && !player_data.stop_run {
            player_data.stop_run = true;
        }
    } else if player_data.stamina < MAX_STAMINA {
        player_data.stamina += STAMINA_RECOVERY_SPEED;

        // if stop_run set and stamine > (value) and player can't run
        // set stop_run = false (player can run)
        if player_data.stamina >= MIN_STAMINA_TO_UNBLOCK_RUN && player_data.stop_run {
            player_data.stop_run = false;
        }
    }
}

/// updates player fear points
pub fn update_fear(
    mut player_query: Query<(&Transform, &mut PlayerComponent)>,
    spheres_query: Query<Option<(&Transform, &SphereOfTear)>>
) {
    // get data
    let (player_transform, mut player) = match player_query.get_single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };

    // player in sphere decrase fear points
    let mut in_sphere = false;
    for sphere in spheres_query.iter() {
        if let Some(sphere) = sphere {
            if sphere.1.point_in_sphere(&sphere.0.translation, &player_transform.translation) {
                in_sphere = true;
                player.fear += FEAR_DECRASE_RATE;
            }
        }
    }

    // if player not in sphere and fear is not minimal
    // recovery fear points
    if !in_sphere && player.fear > 0.1 {
        player.fear -= FEAR_RECOVERY_SPEED;
    }
}

/// update player's rotation
pub fn update_rotation(
    mut model_query: Query<&mut Transform, (With<PlayerComponent>, Without<PlayerCameraPivot>)>,
    player_query: Query<&PlayerControllerData>,
) {
    let Ok(mut model_transform) = model_query.get_single_mut() else { return };
    let Ok(plr_controller) = player_query.get_single() else { return };

    model_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, plr_controller.rotation.y, 0.0);
}

/// move player's kinematic character
pub fn move_character(
    mut player_query: Query<(Entity, &Collider, &mut Transform, &mut PlayerControllerData)>,
    mut pivot_query: Query<&mut Transform, (With<PlayerCameraPivot>, Without<PlayerControllerData>)>,
    mut rapier_context: Query<(
        &mut RapierContextSimulation, &RapierContextColliders, &RapierQueryPipeline, &mut RapierRigidBodySet
    )>,        
    mouse_accumulated_motion: Res<AccumulatedMouseMotion>,
    input_enabled: Res<PlayerInputEnabled>,
    input: Res<PlayersInput>,
    time: Res<Time>,
) {
    if let Err(_) = player_query.get_single_mut() {
        error!("player's not single!");
        return;
    } else if let Err(_) = rapier_context.get_single() {
        error!("rapier context not single!");
        return;
    }

    // get rapier context
    let mut rapier_context = rapier_context.single_mut();

    // get player's data
    let (
        player_entity, player_collider,
        mut player_transform, mut player_controller
    ) = player_query.single_mut();

    let crouch: bool;
    if player_controller.crouched || input.crouch {
        // If there is something above the player and player crouched, then
        // player can't stop crouch
        let mut casts = false;
        let ray_cast_context = (rapier_context.1, rapier_context.2, &rapier_context.3);

        /*
            This cycle check a points and if there is somethins above the player,
            then player can't stop crouch. This is a picture of ray cast, where
            "#" is a player contour, "*" is a ray cast and "@" is a center
                     ####*#####   
                   ##         ##
                  ##           ##
                 ##             ##
                *##      *       #*
                 ##             ##
                  ##           ##
                   ##         ##
                    #####*#####
        */

        for i in 0..5 {
            let mut x_coord: f32 = 0.0;
            let mut z_coord: f32 = 0.0;

            if i == 1 {
                x_coord = PLAYER_RADIUS;
            } else if i == 2 {
                x_coord = -PLAYER_RADIUS;
            } else if i == 3 {
                z_coord = PLAYER_RADIUS;
            } else if i == 4 {
                z_coord = -PLAYER_RADIUS;
            }

            if let Some(_) = crouch_ray_cast(
                ray_cast_context, player_entity,
                &player_transform.translation, x_coord, z_coord
            ) {
                casts = true;
                break;
            }
        }

        // set player crouch at the moment
        crouch = input.crouch || casts;
    } else {
        crouch = false;
    }

    // get mouse delta
    let mouse_delta;
    if input_enabled.0 {
        mouse_delta = mouse_accumulated_motion.delta * 0.0032;
    } else {
        mouse_delta = Vec2::ZERO;
    }

    // calculate new rotation values based on mouse movement
    player_controller.rotation.y = wrap(player_controller.rotation.y - mouse_delta.x, 0.0, 2.0 * PI);
    player_controller.rotation.x = (player_controller.rotation.x - mouse_delta.y).clamp(
        -FRAC_PI_2 + 0.001953125, FRAC_PI_2 - 0.001953125
    );

    // update player's camera rotation
    let Ok(mut pivot_transform) = pivot_query.get_single_mut() else { return; };
    pivot_transform.rotation = Quat::from_euler(EulerRot::XYZ,
        player_controller.rotation.x, 0.0, 0.0
    );

    // get player's speed
    let mut normalized_move = Vec3::ZERO;

    // get player speed
    let player_speed: f32;
    if crouch {
        player_speed = CROUCH_SPEED;
    } else if input.run {
        player_speed = RUN_SPEED;
    } else {
        player_speed = WALK_SPEED;
    }

    // assing initial input directional values
    let mut forward = 0.0;
    let mut sideways = 0.0;

    if input.backward { forward += 1.0 }
    if input.forward { forward -= 1.0 }
    if input.right { sideways += 1.0 }
    if input.left { sideways -= 1.0 }

    let x_fac = player_controller.rotation.y.cos();
    let z_fac = player_controller.rotation.y.sin();

    normalized_move.x = (z_fac * forward) + (x_fac * sideways);
    normalized_move.z = (x_fac * forward) + (-z_fac * sideways);

    // normalize (x and z axis movement)
    normalized_move = normalized_move.normalize_or_zero() * player_speed;

    // update Y - velocity (jump/fly/gravity)
    if player_controller.grounded {
        if player_controller.acceleration.y < 0.0 || player_controller.velocity.y < 0.0 {
            player_controller.acceleration.y = 0.0;
            player_controller.velocity.y = 0.0;
        }

        if input.jump {
            player_controller.velocity.y += player_controller.jump_force;
        }
    } else {
        // update gravity
        if player_controller.velocity.y >= 0.0 {
            player_controller.acceleration.y = -player_controller.gravity;
        }
    }

    // add on acceleration * dt
    let dt = time.delta().as_secs_f32();
    let velocity_change = player_controller.acceleration * dt;
    player_controller.velocity += velocity_change;

    // clamp y-speed to terminal velocity values
    player_controller.velocity.y = player_controller.velocity.y.clamp(
        -player_controller.terminal_velocity, player_controller.terminal_velocity
    );

    // add velocity to move and apply delta time to queued move
    normalized_move += player_controller.velocity;
    normalized_move *= dt;

    // we only want auto-stepping if the player is grounded
    let step: Option<CharacterAutostep>;
    if player_controller.grounded {
        step = Some(CharacterAutostep {
            max_height: CharacterLength::Absolute(1.65),
            min_width: CharacterLength::Absolute(0.5),
            include_dynamic_bodies: true,
        });
    } else {
        step = None;
    };

    // update rotation for crouch
    let rotation: Quat;
    if crouch {
        let needs_rotation =
            Quat::from_rotation_x(player_controller.rotation.y) * Quat::from_rotation_x(90_f32.to_radians());
        rotation = Quat::from_rotation_z(90.0_f32.to_radians()) * needs_rotation;
    } else {
        rotation = Quat::IDENTITY;
    }

    // start move player shape
    let move_output = rapier_context.0.move_shape(
        rapier_context.1, rapier_context.2, &mut *rapier_context.3,
        normalized_move,
        player_collider,
        player_transform.translation,
        rotation,
        player_controller.mass,
        &MoveShapeOptions {
            autostep: step,
            slide: true,
            min_slope_slide_angle: 30.0_f32.to_radians(),
            max_slope_climb_angle: 45.0_f32.to_radians(),
            ..default()
        },
        QueryFilter::new().exclude_collider(player_entity), |_| {}
    );

    // update grounded
    if let Some(_) = rapier_context.2.intersection_with_shape(
        rapier_context.1, &mut *rapier_context.3,
        player_transform.translation + Vec3::new(0.0, -PLAYER_HEIGHT, 0.0),
        Quat::IDENTITY,
        &Collider::cylinder(0.35, PLAYER_RADIUS - 0.1),
        QueryFilter::new().exclude_collider(player_entity)
    ) {
        player_controller.grounded = player_controller.velocity.y <= 0.0;
    } else {
        player_controller.grounded = false;
    }

    // update player position
    player_transform.translation += move_output.effective_translation;

    // If player crouched in last frame and end crouch in this frame player need
    // to up, because if you don't do this player will get stuck
    if player_controller.crouched && !crouch {
        player_transform.translation += Vec3::new(0.0, PLAYER_HEIGHT - PLAYER_RADIUS, 0.0);
    }

    // update crouch state
    player_controller.crouched = input.crouch;
 
    // update cam translation
    if crouch {
        pivot_transform.translation = CAMERA_CROUCH_TRANSLATION;
    } else {
        pivot_transform.translation = CAMERA_WALK_TRANSLATION;
    }   player_controller.crouched = crouch;
}

// helper functions

/// Ray cast for detect something above the player.
/// While he crouch.
#[inline] fn crouch_ray_cast(
     rapier_context: (&RapierContextColliders, &RapierQueryPipeline, &Mut<RapierRigidBodySet>),
     player_entity: Entity,
     translation: &Vec3,
     x_coord: f32,
     z_coord: f32
) -> Option<(Entity, f32)> {
    rapier_context.1.cast_ray(
        rapier_context.0, &*rapier_context.2,
        translation.clone(),
        Vec3::new(x_coord, 1.0, z_coord),
        PLAYER_HEIGHT * 1.2,
        true,
        QueryFilter::new().exclude_collider(player_entity)
    )
}
   
