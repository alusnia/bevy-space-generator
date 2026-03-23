use bevy::camera::visibility::{RenderLayers, calculate_bounds};
use bevy::ecs::spawn;
use bevy::color::LinearRgba;
use bevy::light::NotShadowCaster;
use bevy::math::ops::powf;
use bevy::prelude::*;
// use bevy::render::view::RenderLayers;
// use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::Color;
use bevy::ui::AvailableSpace;
use rand::{RngExt, SeedableRng};
use rand::distr::Alphanumeric;
use rand::rngs::StdRng;

use crate::spawn_orbit_ring;
use crate::{CameraFocusedOn};
use crate::structs::{BelongsTo, LocalStarSystem, OrbitalCamera, SpaceBlueprint, Star, UniverseStar};
use crate::enums::{CameraView, OrbitalBody, StarColor};
use crate::orbital_camera::spawn_orbital_camera;

use std::f32::consts::TAU;

#[derive(Component, Clone)]
pub struct OrbitData {
    pub radius: f32,
    pub angular_speed: f32,
    pub phase_offset: f32,
}

pub fn spawn_star_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected_system: Res<CameraFocusedOn>,
	mut star_query: Query<(&UniverseStar, &Transform, &mut Visibility, &MeshMaterial3d<StandardMaterial>)>,
	camera_query: Query<&OrbitalCamera>
) {
    if let Some(id) = selected_system.entity_id {

        if let Ok((star, transform, mut visibility, mesh_material)) = star_query.get_mut(id) {
            
			let mut rng = StdRng::seed_from_u64(star.seed);

            replace_star(&mut commands, &mut meshes, &mut materials, star, id, transform, &mut visibility, &mut rng);

			let current_camera = camera_query.single().ok();

			spawn_orbital_camera(&mut commands, CameraView::StarSystem, current_camera);

            
        } else {
            println!("Error: no Star with selected id!");
        }
    } else {
        println!("Error: Camera is not focused on anything!");
    }
}


fn replace_star(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    star: &UniverseStar,
	star_id: Entity,
    transform: &Transform,
    visibility: &mut Visibility,
	rng: &mut StdRng
) {
    *visibility = Visibility::Hidden;
	
	let (star_brighness, star_range, frost_line, available_materials) = Star::get_properties(star, rng);

	let (star_id, planets) = spawn_star(commands, meshes, materials, star, star_brighness, star_range, star_id, transform, rng);
	let planet_size = 0.0;
	let blueprints = SpaceBlueprint::get(star, frost_line, available_materials, planets, rng);
	spawn_orbit_ring(commands, meshes, materials, orbit_radius, star_id);
}

fn spawn_star(
	commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
	star: &UniverseStar,
	brightness: f32,
	range: f32,
	star_id: Entity,
	transform: &Transform,
	rng: &mut StdRng
) -> (Entity, usize) {
	let mut planets: usize = draw_planets(rng , 0);
	
	let color = star.color.to_srgb().into();

	let glowing_material = materials.add(StandardMaterial {
		base_color: color,
		emissive: LinearRgba::from(color) * 100.0,
		..default()
	});

	println!("{planets}");

	let mut new_star_id = commands.spawn((
		Star{
			n_of_planets: planets
		},
		LocalStarSystem,
		BelongsTo(star_id),
		Mesh3d(meshes.add(Sphere::new(star.scale))),
		MeshMaterial3d(glowing_material),
		transform.clone(),
		RenderLayers::layer(1),
		NotShadowCaster,
		PointLight{
			color: color,
			intensity: brightness,
			range,
			radius: star.scale,
			shadows_enabled: true,
			..default()
		},
	)).id();
	(new_star_id, planets)
}

fn spawn_planet(
	commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    star: &UniverseStar,
	star_id: Entity,
	planet_index: usize,
	rng: &mut StdRng
) -> (Entity, f32, f32) {

    let (planet_scale, planet_color) = if is_gas_giant {
        (rng.random_range(1.5..3.5), Color::srgb(0.2, 0.4, 0.8))

    } else {
        (rng.random_range(0.4..1.2), Color::srgb(0.7, 0.3, 0.2)) 
    };

    let angular_speed = star.scale / radius.sqrt();
    
    let phase_offset = rng.random_range(0.0..TAU);

    let start_x = radius * phase_offset.cos();
    let start_z = radius * phase_offset.sin();

    let planet_id = commands.spawn((
        Mesh3d(meshes.add(Sphere::new(planet_scale / 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: planet_color,
            ..default()
        })),
        Transform::from_xyz(start_x, 0.0, start_z),
        OrbitData { radius, angular_speed, phase_offset },
        RenderLayers::layer(1), 
    )).id();
	(planet_id, radius, planet_scale)
}

fn draw_planets(
	rng: &mut StdRng,
	iter: u8
) -> usize {
	if iter == 16 {
		return 1;
	}
	let rnd_val = rng.random_range(1..100);
	match rnd_val {
		1..=75 => rnd_val / 15,
		76..=86 => 6,
		87..=92 => 7,
		93..=97 => 8,
		98..=99 => 9,
		_ => 9 + draw_planets(rng, iter + 1),
	}
}

pub fn animate_planets_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &OrbitData)>
) {
    let global_time = time.elapsed_secs();
    
    for (mut transform, orbit) in &mut query {
        let theta = (orbit.angular_speed * global_time) + orbit.phase_offset;
        
        transform.translation.x = orbit.radius * theta.cos();
        transform.translation.z = orbit.radius * theta.sin();
    }
}