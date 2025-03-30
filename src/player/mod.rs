//! In this module located a player logic
//! and structrues

// import crates
use bevy::prelude::*;
use bevy_tnua::math::Float;
use educe::Educe;

// character controller for player
pub(crate) mod character_controller;


/// Struct of player data.
/// There can only be 1 player!
#[derive(Component, Debug, Educe)]
#[require(Transform)]
#[educe(Default)]
pub struct PlayerComponent {
    #[educe(Default = Vec3::NEG_Z)] /// forward look up of player
    pub forward: Vec3,

    #[educe(Default = 0.0)] /// player's pitch angle
    pub pitch_angle: Float,

    #[educe(Default = 0.0)] /// player's fear points
    pub fear: f32,
}

pub mod player_systems {
    //! implementation of player systems
    // import crates
    use bevy::{input::mouse::MouseMotion, prelude::*};
    use avian3d::{math::{AdjustPrecision, Quaternion, FRAC_PI_2}, prelude::*};
    use autodefault::autodefault;

    // import tnua
    use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;
    use bevy_tnua::math::{float_consts::FRAC_PI_4, Float};
    use bevy_tnua::{prelude::*, TnuaToggle};
    use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

    // import character motion config and player component
    use super::character_controller::CharacterMotionConfig;
    use super::PlayerComponent;

    /// Create player
    #[autodefault(except(CharacterMotionConfig))]
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        have_player: Query<Option<&PlayerComponent>>
    ) {
        // if the player already exists, then exit
        if let Ok(_) = have_player.get_single() {
            return;
        }

        // create player struct
        let player = PlayerComponent::default();

        // create player motion config
        let motion_config = CharacterMotionConfig {
            // speeds
            speed: 5.0,
            run_speed: 7.5,
            crouch_speed: 2.5,

            // height's
            height: 0.3, // 1.2,
            crouch_height: 0.01, // 0.8,

            // tnua data
            walk: TnuaBuiltinWalk {
                float_height: 1.2,
                max_slope: FRAC_PI_4 * 4.0,
                turning_angvel: Float::INFINITY
            },
            jump: TnuaBuiltinJump {
                allow_in_air: false,
                height: 2.0
            },
        };

        // player size const's
        const RADIUS: f32 = 0.49;
        const LENGHT: f32 = 1.1;

        // add player With<PlayerComponent>
        commands.spawn(player)
            // The character entity must be configured as a dynamic rigid body of the physics backend.
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule(RADIUS, LENGHT))
            .insert(Transform::from_xyz(0.0, 4.0, 0.0))
            .insert(Mesh3d(meshes.add(Capsule3d::new(RADIUS, LENGHT))))
            .insert(MeshMaterial3d(materials.add(Color::srgb(0.01, 0.87, 0.4))))

            // `TnuaController` is Tnua's main interface with the user code
            .insert(TnuaController::default())
            .insert(motion_config)

            // An entity's Tnua behavior can be toggled individually with this component, if inserted.
            .insert(TnuaToggle::default())
            .insert(TnuaAvian3dSensorShape(Collider::capsule(RADIUS, LENGHT)))

            // lock axes
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())

            // This helper keeps track of air actions like jumps or air dashes.
            .insert(TnuaSimpleAirActionsCounter::default());
    }

    /// Rotate player with mouse input
    pub fn apply_mouse_controls(
        mut mouse_motion: EventReader<MouseMotion>,
        player_character_query: Single<&mut PlayerComponent, (With<PlayerComponent>, Without<Camera>)>,
    ) {
        // get total delta
        let total_delta: Vec2 = mouse_motion.read().map(|event| event.delta).sum();
        let mut player_data = player_character_query.into_inner();

        // calculate yaw and update forward
        let yaw = Quaternion::from_rotation_y(-0.01 * total_delta.x.adjust_precision());
        player_data.forward = yaw.mul_vec3(player_data.forward);

        // calculate pitch and update pithch_angle
        let pitch = 0.005 * total_delta.y.adjust_precision();
        player_data.pitch_angle = (
            player_data.pitch_angle + pitch
        ).clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    use crate::components::SphereOfTear;
    pub fn update_fear(
        player: Single<(&Transform, &mut PlayerComponent)>,
        query: Query<Option<(&Transform, &SphereOfTear)>>
    ) {
        let mut player = player.into_inner();

        for sphere in query.iter() {
            match sphere {
                None => continue,
                Some(sphere) => {
                    if sphere.1.point_in_sphere(&sphere.0.translation, &player.0.translation) {
                        player.1.fear += 1.0;
                        info!("Player in sphere! Fear points: {}", player.1.fear);
                    }
                }
            }
        }
    }
}

