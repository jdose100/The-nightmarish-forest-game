use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

// character controller for player
pub mod character_controller;

#[derive(Component, Debug)]
pub struct PlayerComponent;

#[derive(Component, Debug)]
#[require(Transform)]
pub struct Player {
    front_vec: Vec3 // vector for correct player moving
}

impl Player {
    /// create player system
    pub fn new(mut commands: Commands) {
        // create player struct
        let player = Player {
            front_vec: Vec3::ONE
        };

        // add player With<PlayerComponent>
        commands.spawn((
            PlayerComponent, player,
            Transform::default()
        ));
    }

    /// update player system
    pub fn update(
        player_data: Single<(&mut Transform, &mut Player), With<PlayerComponent>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        accmulated_mouse_input: Res<AccumulatedMouseMotion>,
        time: Res<Time>
    ) {
        let (mut transform, mut player): (Mut<Transform>, Mut<Player>) = player_data.into_inner();

        // rotate player
        const CAMERA_SENSITIVITY: Vec2 = Vec2::new(0.003, 0.002);
        let delta: Vec2 = accmulated_mouse_input.delta;

        if delta != Vec2::ZERO {
            // Note that we are not multiplying by delta_time here.
            // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
            // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
            // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
            // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
            // independent of the framerate.
            let delta_yaw = -delta.x * CAMERA_SENSITIVITY.x;
            let delta_pitch = -delta.y * CAMERA_SENSITIVITY.y;

            let (yaw, pitch, roll): (f32, f32, f32) = transform.rotation.to_euler(EulerRot::YXZ);
            let yaw = yaw + delta_yaw;
            
            // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
            // When the user wants to move the camera back to the horizon, which way should the camera face?
            // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
            // so the direction picked will for all intents and purposes be arbitrary.
            // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
            // To not run into these issues, we clamp the pitch to a safe range.
            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

            player.front_vec = Vec3::new(
                yaw.to_radians().cos() * pitch.to_radians().cos(),
                pitch.to_radians().sin(),
                yaw.to_radians().sin() * pitch.to_radians().cos()
            );
        }

        // move player if keys pressed
        const UP_VECTOR: Vec3 = Vec3::new(0.0, 1.0, 0.0);

        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation -= player.front_vec * time.delta_secs_f64() as f32;
        }
        
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation += player.front_vec * time.delta_secs_f64() as f32;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation -= (player.front_vec.cross(UP_VECTOR) * time.delta_secs_f64() as f32).normalize();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation += (player.front_vec.cross(UP_VECTOR) * time.delta_secs_f64() as f32).normalize();
        }        
    }
}

