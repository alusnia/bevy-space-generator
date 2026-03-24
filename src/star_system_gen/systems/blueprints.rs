use rand::rngs::StdRng;

use crate::universe_gen::components::UniverseStar;
use crate::star_system_gen::components::{SpaceBlueprint, OrbitalBody};


pub fn sketch(
	star: &UniverseStar,
	frost_line: f32,
	available_materials: f32,
	planets: usize,
	rng: &mut StdRng
	) -> Vec<SpaceBlueprint> {
		let mut vector: Vec<SpaceBlueprint> = Vec::with_capacity(planets);
		let mut orbit = 0.0;
		let mut frozen_giants = false;
		for x_planet in 0..planets {
			orbit = Self::calculate_orbit(star, orbit, 0.0, rng);
			let body_type = OrbitalBody::get(orbit < frost_line, frozen_giants, orbit / (2.0*frost_line) ,rng);
			if body_type == OrbitalBody::IceGiant {frozen_giants = true};
			let tokens = OrbitalBody::get_tokens(&body_type, rng);
			vector.push(SpaceBlueprint {orbit, tokens, gravity: 0, body_type, is_super: false, n_of_moon: 0 });
		}
		for x_planet in 0..planets {
			
		}
		vector
	}