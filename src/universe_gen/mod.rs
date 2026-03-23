use bevy::prelude::*;

pub mod components;
pub mod math;
pub mod systems;

use systems::spawn_universe;

pub struct UniverseGenPlugin;

impl Plugin for UniverseGenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_universe)
        ;
    }
}