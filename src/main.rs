#![allow(unused)]
use bevy::camera::CameraPlugin;
use bevy::prelude::*;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::universe_gen::UniverseGenPlugin;
use crate::star_system_gen::StarSystemPlugin;
use crate::vfx::VFXPlugin;

mod camera;
mod vfx;
mod player;
mod star_system_gen;
mod universe_gen;

#[derive(Component)]
pub struct BelongsTo(pub Entity);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    GalaxyMap,
    StarSystem,
	PlanetSystem,
	Planet,
}

#[derive(Resource)]
struct GameRng(pub StdRng);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
		.add_plugins(CameraPlugin)
		.add_plugins(VFXPlugin)
		.add_plugins(UniverseGenPlugin)
		.add_plugins(StarSystemPlugin)
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            brightness: 20.0,
            ..default() 
        })
		.insert_resource(GameRng(StdRng::seed_from_u64(2137420)))
		.init_state::<GameState>()
        .run();
}
