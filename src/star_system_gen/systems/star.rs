use bevy::prelude::*;
use bevy::camera::visibility::{RenderLayers};
use bevy::light::NotShadowCaster;
use rand::rngs::StdRng;
use rand::RngExt;

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
