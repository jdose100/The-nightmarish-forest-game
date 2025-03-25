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


// import this crate
use crate::systems::setup;
use crate::player::Player;
use crate::player::character_controller::CharacterControllerPlugin;
use crate::camera::CameraComponent;


// add game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // add dev plugins
        if cfg!(debug_assertions) {
            app.add_plugins(WorldInspectorPlugin::new());
            app.add_plugins(PhysicsDebugPlugin::new(FixedUpdate));
        }

        // add plugins
        app.add_plugins(PhysicsPlugins::new(FixedUpdate));        
        app.add_plugins(CharacterControllerPlugin);

        // add resources

        // add events

        // add systems
        app.add_systems(Startup, crate::setup);
        app.add_systems(Update,
            CameraComponent::update_with_plr.before(Player::apply_mouse_controls)
        );
    }
}

