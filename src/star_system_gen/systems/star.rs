use bevy::prelude::*;
use bevy::camera::visibility::{RenderLayers};
use bevy::light::NotShadowCaster;
use rand::rngs::StdRng;
use rand::RngExt;
use std::f32::consts::TAU;

use crate::universe_gen::components::UniverseStar;
use crate::star_system_gen::components::{LocalStarSystem, OrbitData, SpaceBlueprint, Star, SystemProperties, OrbitalBody};
use crate::star_system_gen::math::{draw_planets, get_properties};
use crate::BelongsTo;

pub fn replace_star(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    star: &UniverseStar,
	star_id: Entity,
    transform: &Transform,
    visibility: &mut Visibility,
	rng: &mut StdRng
) -> SystemProperties {
    *visibility = Visibility::Hidden;
	
	let mut system = get_properties(star, rng);

	system.planets = draw_planets(rng , 0);
	
	let color = star.color.to_srgb().into();

	let glowing_material = materials.add(StandardMaterial {
		base_color: color,
		emissive: LinearRgba::from(color) * 100.0,
		..default()
	});

	system.star_id = Some(commands.spawn((
		Star{
			n_of_planets: system.planets
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
			intensity: system.brightness,
			range: system.range,
			radius: star.scale,
			shadows_enabled: true,
			..default()
		},
	)).id());
	system
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