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

pub struct StarData {
	pub min_temp: u16,
	pub max_temp: u16,
	pub min_scale: f32,
	pub max_scale: f32,
	pub color: StarColor,
}