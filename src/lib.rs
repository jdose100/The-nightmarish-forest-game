//! The nightmarish fores is a horror game, this module store
//! a all game logis of this game

// import crates
use bevy::prelude::*;
use avian3d::prelude::*;

// import crates if dev build
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;


// add modules
pub(crate) mod player;
pub(crate) mod systems;
pub(crate) mod camera;
pub(crate) mod components;


// import this crate
use crate::systems::setup;
use crate::player::player_systems;
use crate::player::character_controller::CharacterControllerPlugin;
use crate::camera::camera_systems as cam_systems;

/// A main game logic plugin, this plugin
/// add all systems of game
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // add dev plugins
        if cfg!(debug_assertions) {
            app.add_plugins(WorldInspectorPlugin::new());
            // app.add_plugins(PhysicsDebugPlugin::new(FixedUpdate));
        }

        // add plugins
        app.add_plugins(PhysicsPlugins::new(FixedUpdate));        
        app.add_plugins(CharacterControllerPlugin);

        // add resources

        // add events

        // add systems
        app.add_systems(Startup, crate::setup);
        app.add_systems(Update,
            cam_systems::update_with_plr.before(player_systems::apply_mouse_controls)
        );

        app.add_systems(Update, player_systems::update_fear);
    }
}

