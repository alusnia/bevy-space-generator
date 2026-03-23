use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use crate::camera::components::{CameraFocusedOn, OrbitalCamera, CameraView};
use crate::camera::systems::spawn_orbital_camera;
use crate::universe_gen::components::UniverseStar;
use crate::star_system_gen::systems::star::replace_star;

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

            let system = replace_star(&mut commands, &mut meshes, &mut materials, star, id, transform, &mut visibility, &mut rng);

			let current_camera = camera_query.single().ok();

			spawn_orbital_camera(&mut commands, CameraView::StarSystem, current_camera);

			let blueprints = make_blueprints(system);
			spawn_orbit_ring(commands, meshes, materials, orbit_radius, star_id);
        } else {
            println!("Error: no Star with selected id!");
        }
    } else {
        println!("Error: Camera is not focused on anything!");
    }
}