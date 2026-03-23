#![allow(unused)]
use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use rand::{RngExt, SeedableRng};
use rand::distr::Alphanumeric;
use rand::rngs::StdRng;
use crate::structs::OrbitalCamera;

mod core_graphics;
mod player;
mod star_system_gen;
mod universe_gen;

use enums::CameraView;
use structs::{LocalStarSystem, UniverseStar};
use universe::spawn_universe;
use orbital_camera::orbital_camera_system;
use star_system::{spawn_star_system, animate_planets_system};
use visual_effects::{dynamic_lens_exposure, dynamic_star_scaling};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    GalaxyMap,
    StarSystem,
	PlanetSystem,
	Planet,
}

#[derive(Resource)]
pub struct CameraFocusedOn { pub entity_id: Option<Entity> }

#[derive(Resource)]
struct GameRng(pub StdRng);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            brightness: 20.0,
            ..default() 
        })
		.insert_resource(GameRng(StdRng::seed_from_u64(2137420)))
		.insert_resource(CameraFocusedOn{entity_id: None})
		.init_state::<GameState>()
        .add_systems(Startup, spawn_universe)
        .add_systems(Update, (
			orbital_camera_system,
			dynamic_lens_exposure,
			dynamic_star_scaling,
			animate_planets_system
		))
		.add_systems(OnEnter(GameState::GalaxyMap), exit_star_system)
		.add_systems(OnEnter(GameState::StarSystem), spawn_star_system)
        .run();
}

pub fn exit_star_system(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &CameraView, &mut OrbitalCamera)>,
    local_star_query: Query<Entity, With<LocalStarSystem>>,
    mut universe_star_query: Query<&mut Visibility, With<UniverseStar>>,
) {
    for (entity, view, mut orbit) in &mut camera_query {
        if *view == CameraView::StarSystem {
            commands.entity(entity).despawn();
        } else if *view == CameraView::Galaxy {

            orbit.target_radius = orbit.min_radius + 20.0;
            orbit.radius = orbit.target_radius;
        }
    }
    
    for entity in &local_star_query { commands.entity(entity).despawn(); }
    for mut visibility in &mut universe_star_query { *visibility = Visibility::Inherited; }
}

pub fn spawn_orbit_ring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    radius: f32,
    center_entity: Entity,
) {
    let ring_mesh = meshes.add(Torus { 
        minor_radius: 0.008,
        major_radius: radius,
    }.mesh()
	.minor_resolution(6)
	.major_resolution(256)
	);

    let ring_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.8, 0.8, 1.0, 0.25),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let ring_id = commands.spawn((
        Mesh3d(ring_mesh),
        MeshMaterial3d(ring_material),
        Transform::default(),
        RenderLayers::layer(1),
    )).id();

    commands.entity(center_entity).add_child(ring_id);
}