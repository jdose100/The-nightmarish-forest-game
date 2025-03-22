// import crates
use bevy::prelude::*;
use avian3d::prelude::*;

// import crates if dev build
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;


// add modules
pub mod player;
pub mod systems;


// import this crate
use crate::systems::{setup, camera_update};
use crate::player::Player;
use crate::player::character_controller::CharacterControllerPlugin;


// add game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // add dev plugins
        #[cfg(debug_assertions)]
        app.add_plugins(WorldInspectorPlugin::new());

        // add plugins
        app.add_plugins((
                PhysicsPlugins::default(),
                CharacterControllerPlugin
        ));

        // add resources

        // add events

        // add systems
        app.add_systems(Startup, (crate::setup));
        // app.add_systems(Update, camera_update);
        // app.add_systems(Update, (crate::camera_update, Player::update).chain());
    }
}

