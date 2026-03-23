use bevy::prelude::*;

pub mod components;
pub mod math;
pub mod systems;

use crate::star_system_gen::systems::{spawn_star_system};
use crate::GameState;
use systems::{orbital_camera_system, exit_star_system};
use components::CameraFocusedOn;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, orbital_camera_system)
			.add_systems(OnEnter(GameState::GalaxyMap), exit_star_system)
			.add_systems(OnEnter(GameState::StarSystem), spawn_star_system)
			.insert_resource(CameraFocusedOn{entity_id: None});
    }
}