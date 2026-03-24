use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::RngExt;

#[derive(Component, Clone)]
pub struct OrbitData {
    pub radius: f32,
    pub angular_speed: f32,
    pub phase_offset: f32,
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