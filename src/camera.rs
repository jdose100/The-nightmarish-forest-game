//! Controll camera systems

// import crates
use bevy::prelude::*;

/// main camera in game
#[derive(Component)]
#[require(Transform, Camera3d)]
pub struct CameraComponent {}

pub mod camera_systems {
    //! implementation of camera systems
    // import crates
    use bevy::prelude::*;
    use bevy_tnua::math::AsF32;
    use crate::player::PlayerComponent;

    /// Update camera transform with player
    pub fn update_with_plr(        
        player_character_query: Single<(&GlobalTransform, &PlayerComponent), (With<PlayerComponent>, Without<Camera>)>,
        mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerComponent>)>
    ) {
        let (player_transform, player_data) = player_character_query.into_inner();
        for mut camera in camera_query.iter_mut() {
            camera.translation = player_transform.translation() + -5.0 * player_data.forward.f32() + 1.0 * Vec3::Y;
            camera.look_to(player_data.forward.f32(), Vec3::Y);

            let pitch_axis = camera.left();
            camera.rotate_around(player_transform.translation(),
                Quat::from_axis_angle(*pitch_axis, player_data.pitch_angle.f32())
            );
        }
    }
}

