//! In this module located a player logic
//! and structrues

// import crates
use bevy::prelude::*;

// character controller for player
pub(super) mod systems;
pub mod structures;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(structures::PlayersInput::default());
        app.insert_resource(structures::PlayerInputEnabled(false));

        app.add_systems(Startup, systems::setup);
        app.add_systems(Update, (
           systems::update_input, systems::update_cursor_visible, systems::update_fear,
           systems::update_stamina,
           (systems::move_character, systems::update_rotation, systems::update_cursor_position).chain() 
        ));
    }
}

