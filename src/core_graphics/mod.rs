use bevy::prelude::*;

pub mod components;
pub mod math;
pub mod systems;

pub struct UniverseGenPlugin;

impl Plugin for UniverseGenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::generate_and_spawn_universe)
        ;
    }
}