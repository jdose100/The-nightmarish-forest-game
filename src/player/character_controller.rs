//! Character controller systems and plugin for player

// import avian 3d
use avian3d::math::AdjustPrecision;

// import bevy
use bevy::prelude::*;

// import bevy-tnua
use bevy_tnua::prelude::*;
use bevy_tnua::math::{AsF32, Float, Vector3};
use bevy_tnua::control_helpers::{TnuaCrouchEnforcerPlugin, TnuaSimpleAirActionsCounter};
use bevy_tnua_avian3d::*;

// import this crate
use super::PlayerComponent;
use super::player_systems;

/// Plugins add character controller for player
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        // add plugins
        app.add_plugins(TnuaAvian3dPlugin::new(FixedUpdate));

        // this is Tnua's main plugin
        app.add_plugins(TnuaControllerPlugin::new(FixedUpdate));
        app.add_plugins(TnuaCrouchEnforcerPlugin::new(FixedUpdate));

        // add systems
        app.add_systems(Startup, player_systems::setup);
        app.add_systems(PostUpdate,
            player_systems::apply_mouse_controls.before(bevy::transform::TransformSystem::TransformPropagate)
        );
        app.add_systems(FixedUpdate, apply_platformer_controls.in_set(TnuaUserControlsSystemSet));
    }
}

/// Character motion config
#[derive(Component)]
pub struct CharacterMotionConfig {
    /// speed in walk mode
    pub speed: Float,

    /// speed in run mode
    pub run_speed: Float,

    /// speed in crouch mode
    pub crouch_speed: Float,

    /// growth while sitting
    pub crouch_height: Float, 

    /// growth in normal condition
    pub height: Float, 

    /// tnua's walk data
    pub walk: TnuaBuiltinWalk,

    /// tnua's jump data
    pub jump: TnuaBuiltinJump,
}

// constants for character control
struct MoveKeys { forward: KeyCode, back: KeyCode, left: KeyCode, right: KeyCode, jump: KeyCode }

const CROUCH_KEYS: [KeyCode; 2] = [KeyCode::ControlLeft, KeyCode::ControlRight];
const RUN_KEYS: [KeyCode; 2] = [KeyCode::ShiftLeft, KeyCode::ShiftRight];
const MOVE_KEYS: MoveKeys = MoveKeys {
    forward: KeyCode::KeyW,
    back: KeyCode::KeyS,
    left: KeyCode::KeyA,
    right: KeyCode::KeyD,
    jump: KeyCode::Space
};

/// Exercising character control
pub fn apply_platformer_controls(
   keyboard: Res<ButtonInput<KeyCode>>,
   mut query: Query<(
       &CharacterMotionConfig,
       &mut TnuaController,
       &mut TnuaSimpleAirActionsCounter,
       &PlayerComponent
   )>
) {
    for (
        config, mut controller,
        mut air_actions_counter, player_component
    ) in query.iter_mut() {
        let mut direction = Vector3::ZERO;

        // get direction from keyboadrd
        if keyboard.pressed(MOVE_KEYS.forward) {
            direction -= Vector3::Z;
        }

        if keyboard.pressed(MOVE_KEYS.back) {
            direction += Vector3::Z;
        }

        if keyboard.pressed(MOVE_KEYS.left) {
            direction -= Vector3::X;
        }

        if keyboard.pressed(MOVE_KEYS.right) {
            direction += Vector3::X;
        }

        // calculate direction
        direction = direction.clamp_length_max(1.0);
        direction = bevy::prelude::Transform::default()
                .looking_to(player_component.forward.f32(), Vec3::Y)
                .transform_point(direction.f32())
                .adjust_precision();

        // get active keys
        let crouch = keyboard.any_pressed(CROUCH_KEYS);
        let jump = keyboard.pressed(MOVE_KEYS.jump);
        let run = keyboard.any_pressed(RUN_KEYS);

        // This needs to be called once per frame. It lets the air actions counter know about the
        // air status of the character. Specifically:
        // * Is it grounded or is it midair?
        // * Did any air action just start?
        // * Did any air action just finished?
        // * Is any air action currently ongoing?
        air_actions_counter.update(controller.as_mut());


        // get player speed
        let speed =
            if crouch {
                config.crouch_speed
            } else if run {
                config.run_speed
            } else {
                config.speed
            };

        // get player height
        let height = if crouch { config.crouch_height } else { config.height };

        // The basis is Tnua's most fundamental control command, governing over the character's
        // regular movement. The basis (and, to some extent, the actions as well) contains both
        // configuration - which in this case we copy over from `config.walk` - and controls like
        // `desired_velocity` or `desired_forward` which we compute here based on the current
        // frame's input.
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction * speed,
            float_height: height,

            // With shooters, we want the character model to follow the camera.
            desired_forward: Dir3::new(player_component.forward.f32()).ok(),
            ..config.walk.clone()
        });

        if jump && !crouch {
            controller.action(TnuaBuiltinJump {
                // Jumping, like crouching, is an action that we either feed or don't. However,
                // because it can be used in midair, we want to set its `allow_in_air`. The air
                // counter helps us with that.
                //
                // The air actions counter is used to decide if the action is allowed midair by
                // determining how many actions were performed since the last time the character
                // was considered "grounded" - including the first jump (if it was done from the
                // ground) or the initiation of a free fall.
                //
                // `air_count_for` needs the name of the action to be performed (in this case
                // `TnuaBuiltinJump::NAME`) because if the player is still holding the jump button,
                // we want it to be considered as the same air action number. So, if the player
                // performs an air jump, before the air jump `air_count_for` will return 1 for any
                // action, but after it it'll return 1 only for `TnuaBuiltinJump::NAME`
                // (maintaining the jump) and 2 for any other action. Of course, if the player
                // releases the button and presses it again it'll return 2.
                allow_in_air:
                    air_actions_counter.air_count_for(TnuaBuiltinJump::NAME) <= 0,
                ..config.jump.clone()
            });
        }
    }
}

