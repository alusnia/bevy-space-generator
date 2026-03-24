use rand::rngs::StdRng;

use crate::universe_gen::components::UniverseStar;
use crate::star_system_gen::components::{SpaceBlueprint, OrbitalBody};
use crate::star_system_gen::math::{calculate_orbit, get_tokens};

pub fn sketch(
	star: &UniverseStar,
	frost_line: f32,
	available_materials: f32,
	planets: usize,
	rng: &mut StdRng
	) -> (Vec<SpaceBlueprint>, u32 {
		let mut vector: Vec<SpaceBlueprint> = Vec::with_capacity(planets);
		let mut orbit = 0.0;
		let mut frozen_giants = false;
		let mut tokens_sum = 0;
		for x_planet in 0..planets {
			let orbit = calculate_orbit(star, orbit, 0.0, rng);
			let body_type = OrbitalBody::get(orbit < frost_line, frozen_giants, orbit / (2.0*frost_line) ,rng);
			if body_type == OrbitalBody::IceGiant {frozen_giants = true};
			let tokens = get_tokens(&body_type, rng);
			tokens_sum += tokens;
			let get_size = get_size(tokens, available_materials);
			vector.push(SpaceBlueprint { orbit, tokens, gravity: 0, body_type, is_super: false, n_of_moons: 0});
		}
		(vector, materials_per_token(tokens_sum, available_materials))
	}

	pub fn materials_per_token(
		tokens_sum: u32,
		available_materials: f32
	) -> f32 {
		available_materials / tokens_sum as f32
	}