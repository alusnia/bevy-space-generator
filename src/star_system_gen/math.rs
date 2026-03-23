use bevy::math::ops::powf;
use rand::rngs::StdRng;
use rand::RngExt;

use crate::star_system_gen::components::{OrbitalBody, SystemProperties};
use crate::universe_gen::components::UniverseStar;

pub fn draw_planets(
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

pub fn get_tokens(
	body_type: &OrbitalBody,
	rng: &mut StdRng
) -> u32 {
	match body_type {
		OrbitalBody::Planetoid => rng.random_range(1..20),
		OrbitalBody::AsteroidBelt => rng.random_range(15..60),
		OrbitalBody::Rocky => rng.random_range(50..400),
		OrbitalBody::IceGiant => rng.random_range(1000..4000),
		OrbitalBody::GasGiant => rng.random_range(8000..30000)
	}
}

pub fn calculate_orbit(
	star: &UniverseStar,
	previous_orbit: f32,
	planet_size: f32,
	rng: &mut StdRng
) -> f32 {
	match previous_orbit {
		..=0.0 => star.scale * rng.random_range(2.5..3.0),
		_ => {
			let mut temp= previous_orbit * rng.random_range(1.4..1.9);
			temp + powf(planet_size, 2.0) * 10.0
		},
	}
}

pub fn get_properties(
	star: &UniverseStar,
	rng: &mut StdRng
) -> SystemProperties {
	const BRIGHTNESS_MUL: f32 = 300.0;

	const RANGE_MUL: f32 = 100.0;

	const FROST_BASE: f32 = 100.0;

	let base_val = star.scale * (star.temperature / 5778).pow(2) as f32;

	let brightness = powf(base_val, 2.0) * BRIGHTNESS_MUL;

	let range = base_val * RANGE_MUL;

	let frost_line = base_val * FROST_BASE;

	let available_materials = rng.random_range(0.0..=2.0) * star.scale;

	SystemProperties { star_id: None, brightness, range, frost_line, available_materials, planets: 0 }
}