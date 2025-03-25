// import crates
use bevy::{input::mouse::MouseMotion, prelude::*};
use avian3d::{math::{AdjustPrecision, Quaternion, FRAC_PI_2}, prelude::*};
use bevy_tnua::control_helpers::{TnuaCrouchEnforcer, TnuaSimpleAirActionsCounter, TnuaSimpleFallThroughPlatformsHelper};
use bevy_tnua::math::{float_consts::FRAC_PI_4, Float, Vector3};
use bevy_tnua::{prelude::*, TnuaGhostSensor, TnuaToggle};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use autodefault::autodefault;
use educe::Educe;

// character controller for player
pub(crate) mod character_controller;

// import character controller
use self::character_controller::CharacterMotionConfig;


#[derive(Component, Debug)]
pub struct PlayerComponent;

/// struct with player data
#[derive(Component, Debug, Educe)]
#[require(Transform)]
#[educe(Default)]
pub struct Player {
    #[educe(Default = Vec3::NEG_Z)]
    pub(crate) forward: Vec3,

    #[educe(Default = 0.0)]
    pub(crate) pitch_angle: Float
}

impl Player {
    /// create player system
    #[autodefault(except(CharacterMotionConfig))]
    pub(crate) fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>
    ) {
        // create player struct
        let player = Player::default();

        // create player motion config
        let motion_config = CharacterMotionConfig {
            // speeds
            speed: 5.0,
            run_speed: 7.5,
            crouch_speed: 2.5,

            // height's
            height: 1.2,
            crouch_height: 0.8,

            // tnua data
            walk: TnuaBuiltinWalk {
                float_height: 1.2,
                max_slope: FRAC_PI_4 * 4.0,
                turning_angvel: Float::INFINITY
            },
            jump: TnuaBuiltinJump { height: 3.0 },

            // other data
            actions_in_air: 1,
        };

        // player size const's
        const RADIUS: f32 = 0.5;
        const LENGHT: f32 = 1.0;

        // add player With<PlayerComponent>
        commands.spawn(PlayerComponent)
            // The character entity must be configured as a dynamic rigid body of the physics backend.
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule(RADIUS, LENGHT))
            .insert(Transform::from_xyz(0.0, 4.0, 0.0))
            .insert(Mesh3d(meshes.add(Capsule3d::new(RADIUS, LENGHT))))
            .insert(MeshMaterial3d(materials.add(Color::srgb(0.01, 0.87, 0.4))))

            // `TnuaController` is Tnua's main interface with the user code
            .insert(TnuaController::default())
            .insert(motion_config)
            
            // add player data
            .insert(player)

            // An entity's Tnua behavior can be toggled individually with this component, if inserted.
            .insert(TnuaToggle::default())        

            // let layers = [LayerNames::Default, LayerNames::Player];
            .insert(TnuaCrouchEnforcer::new(0.5 * Vector3::Y, |cmd| {
                cmd.insert(TnuaAvian3dSensorShape(Collider::cylinder(0.5, 0.0)));
            }))

            // The ghost sensor is used for detecting ghost platforms - platforms configured in the physics
            // backend to not contact with the character (or detect the contact but not apply physical
            // forces based on it) and marked with the `TnuaGhostPlatform` component. These can then be
            // used as one-way platforms.
            .insert(TnuaGhostSensor::default())

            // This helper is used to operate the ghost sensor and ghost platforms and implement
            // fall-through behavior where the player can intentionally fall through a one-way platform.
            .insert(TnuaSimpleFallThroughPlatformsHelper::default())

            // This helper keeps track of air actions like jumps or air dashes.
            .insert(TnuaSimpleAirActionsCounter::default());
    }

    /// rotate player with mouse input
    pub(crate) fn apply_mouse_controls(
        mut mouse_motion: EventReader<MouseMotion>,
        player_character_query: Single<&mut Player, (With<PlayerComponent>, Without<Camera>)>,
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
}

