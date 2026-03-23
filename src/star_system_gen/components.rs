use bevy::prelude::*;
use std::option::Option;

use crate::universe_gen::components::{UniverseStar, StarColor};
use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;

#[derive(Component)]
pub struct LocalStarSystem;

#[derive(Component)]
pub struct Star {
	pub n_of_planets: usize
}

pub struct SystemProperties {
    pub star_id: Option<Entity>,
    pub brightness: f32,
    pub range: f32,
    pub frost_line: f32,
    pub available_materials: f32,
    pub planets: usize,
}

#[derive(Component, Clone)]
pub struct OrbitData {
    pub radius: f32,
    pub angular_speed: f32,
    pub phase_offset: f32,
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
		for x_planet in 0..planets {
			
		}
		vector
	}
}

#[derive(PartialEq)]
pub enum OrbitalBody {
	Planetoid,
	AsteroidBelt,
	Rocky,
	IceGiant,
	GasGiant
}

impl OrbitalBody {
	pub fn get(
		warm: bool,
		frozen_giants: bool,
		multiplier: f32,
		rng: &mut StdRng,
	) -> OrbitalBody {
		if warm {
			match rng.random_range(0..1000) {
				0..=100 => OrbitalBody::AsteroidBelt,
				101..=995 => OrbitalBody::Rocky,
				_ => OrbitalBody::GasGiant,
			}
		}
		else if !frozen_giants {
			match rng.random_range(0.0..100.0) * multiplier{
				0.0..=25.0 => OrbitalBody::AsteroidBelt,
				25.0..=80.0 => OrbitalBody::GasGiant,
				_ => OrbitalBody::IceGiant,
			}
		}
		else {
			match rng.random_range(0..100) {
				0..=30 => OrbitalBody::AsteroidBelt,
				31..=80 => OrbitalBody::Planetoid,
				_ => OrbitalBody::IceGiant,
			}
		}
	}
}