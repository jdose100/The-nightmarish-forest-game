//! This module store player's structures, enums and consts
use bevy::prelude::*;
use educe::Educe;

// const's
// player size
pub(super) const PLAYER_HEIGHT: f32 = 1.5;
pub(super) const PLAYER_RADIUS: f32 = 1.0;

// player stamina
pub(super) const MIN_STAMINA_TO_UNBLOCK_RUN: f32 = 52.0;
pub(super) const STAMINA_RECOVERY_SPEED: f32 = 0.3;
pub(super) const STAMINA_DECRASE_RATE: f32 = 0.7;
pub(super) const MAX_STAMINA: f32 = 250.0 * 10000.0;

// player speed
pub(super) const CROUCH_SPEED: f32 = 0.7;
pub(super) const WALK_SPEED: f32 = 2.0;
pub(super) const RUN_SPEED: f32 = 4.72;

// player's fear
pub(super) const FEAR_RECOVERY_SPEED: f32 = 0.05;
pub(super) const FEAR_DECRASE_RATE: f32 = 0.12;

// camera translation offsets
pub(crate) const CAMERA_WALK_TRANSLATION: Vec3 = Vec3::new(0.0, 1.5, 0.0);
pub(crate) const CAMERA_CROUCH_TRANSLATION: Vec3 = Vec3::ZERO;

/// Struct of player data.
/// There can only be 1 player!
#[derive(Component, Debug, Educe)]
#[require(Transform)]
#[educe(Default)]
pub struct PlayerComponent {
    #[educe(Default = 0.0)] /// player's fear points
    pub fear: f32,

    #[educe(Default = MAX_STAMINA)] /// player's current stamina
    pub stamina: f32,

    #[educe(Default = false)] /// if stop_run == true player can't run
    pub stop_run: bool,
}

#[derive(Component, Default)] /// player's camera pivot
pub struct PlayerCameraPivot;

#[derive(Resource)] /// check player input or not
pub struct PlayerInputEnabled(pub bool);

#[derive(Resource, Default)] /// player's input data
pub struct PlayersInput {
    // directional
    pub backward: bool,
    pub forward: bool,
    pub right: bool,
    pub left: bool,

    // action's
    pub jump: bool,
    pub run: bool,
    pub crouch: bool,
}

#[derive(Component, Educe, Debug)]
#[educe(Default)] /// player's data for controller
pub struct PlayerControllerData {
    #[educe(Default = 7.0)] /// player's jump force
    pub jump_force: f32,

    #[educe(Default = false)] /// player's crouch in the last frame
    pub crouched: bool,

    // physics data
    #[educe(Default = Vec3::ZERO)] /// player's acceleration
    pub acceleration: Vec3,

    #[educe(Default = Vec3::ZERO)] /// player's velocity
    pub velocity: Vec3,

    #[educe(Default = Vec2::ZERO)] /// player's rotation
    pub rotation: Vec2,

    #[educe(Default = 100.0)] /// player's mass
    pub mass: f32,

    #[educe(Default = 9.8)] /// player's gravity strenght
    pub gravity: f32,

    #[educe(Default = 180.0)] /// player's max velocity
    pub terminal_velocity: f32,

    #[educe(Default = false)] /// player on ground, or not
    pub grounded: bool,
}

