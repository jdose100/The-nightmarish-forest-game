//! The nightmarish fores is a horror game, this module store
//! a all game logis of this game

// import crates
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_skein::SkeinPlugin;
use bevy_rapier3d::prelude::*;

// import crates if dev build
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// add modules
pub(crate) mod components;
pub(crate) mod systems;
pub(crate) mod player;
pub(crate) mod camera;
pub(crate) mod ui;

/// A main game logic plugin, this plugin
/// add all systems of game
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // add dev plugins
        if cfg!(debug_assertions) {
            app.add_plugins(WorldInspectorPlugin::new());
            // app.add_plugins(RapierDebugRenderPlugin::default());
        }

        // add plugins
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            FrameTimeDiagnosticsPlugin,
            player::PlayerPlugin,
            SkeinPlugin::default(),
            MeshPickingPlugin,
        ));

        // register types
        app.register_type::<components::SphereOfTear>();

        // add resources

        // add systems
        app.add_systems(Startup, (
            systems::setup_world, ui::setup_gui
        ));

        app.add_systems(Update, (ui::update_gui_text, systems::in_inventory));
    }
}

