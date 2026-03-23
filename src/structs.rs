use bevy::{math::ops::powf, post_process::bloom::BloomPlugin, prelude::*, ui::AvailableSpace};
use rand::{RngExt, rngs::StdRng};
use crate::enums::{StarColor, StarType, OrbitalBody};

#[derive(Component)]
pub struct BelongsTo(pub Entity);

#[derive(Component, Clone)]
pub struct OrbitalCamera {
    pub focus: Vec3,
    pub target_focus: Vec3,
    pub radius: f32,
    pub target_radius: f32,
    pub yaw: f32,
    pub pitch: f32,
	pub min_radius: f32, 
    pub max_radius: f32,
}

#[derive(Component)]
pub struct Universe { pub name: String }

#[derive(Component)]
pub struct UniverseStar {
	pub name: String,
	pub temperature: u32,
	pub scale: f32,
	pub seed: u64,
	pub color: StarColor,
	pub kind: StarType
}

#[derive(Component)]
pub struct LocalStarSystem;

#[derive(Component)]
pub struct Star {
	pub n_of_planets: usize
}

impl Star {
	pub fn get_properties(
		star: &UniverseStar,
		rng: &mut StdRng
	) -> (f32, f32, f32, f32) {
		const BRIGHTNESS_MUL: f32 = 300.0;

		const RANGE_MUL: f32 = 100.0;

		const FROST_BASE: f32 = 100.0;

		let base_val = star.scale * (star.temperature / 5778).pow(2) as f32;

		let brightness = powf(base_val, 2.0) * BRIGHTNESS_MUL;

		let range = base_val * RANGE_MUL;

		let frost_line = base_val * FROST_BASE;

		let available_materials = rng.random_range(0.0..=2.0) * star.scale;

		(brightness, range, frost_line, available_materials)
	}
}

pub struct StarData {
	pub min_temp: u16,
	pub max_temp: u16,
	pub min_scale: f32,
	pub max_scale: f32,
	pub color: StarColor,
}

pub struct SpaceBlueprint {
	pub orbit: f32,
	pub tokens: u32,
	pub gravity: u32,
	pub body_type: OrbitalBody,
	pub is_super: bool,
	pub n_of_moon: u8,
}

impl SpaceBlueprint{
	pub fn get(
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
		vector
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
}

pub struct Chunk {
	pub has_star: bool,
	pub position: (f32, f32, f32)
}