use bevy::prelude::*;
use rand::{RngExt, rngs::StdRng};
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum CameraView {
    Galaxy,
    StarSystem,
}

pub enum StarColor {
	Red,
	Orange,
	Yellow,
	White,
	Blue,
}

impl StarColor {
	pub fn to_linear_rgba(&self) -> LinearRgba {
		match self {
			StarColor::Red => LinearRgba::rgb(1.0, 0.2, 0.2),
            StarColor::Orange => LinearRgba::rgb(1.0, 0.6, 0.2),
            StarColor::Yellow => LinearRgba::rgb(1.0, 1.0, 0.6),
            StarColor::White => LinearRgba::rgb(0.8, 0.9, 1.0),
            StarColor::Blue => LinearRgba::rgb(0.4, 0.6, 1.0),
		}
	}
	
	pub fn to_srgb(&self) -> Color {
		match self {
			StarColor::Red => Color::srgb(1.0, 0.2, 0.2),
            StarColor::Orange => Color::srgb(1.0, 0.6, 0.2),
            StarColor::Yellow => Color::srgb(1.0, 1.0, 0.6),
            StarColor::White => Color::srgb(0.8, 0.9, 1.0),
            StarColor::Blue => Color::srgb(0.4, 0.6, 1.0),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum StarType {
    RedDwarf,
    OrangeDwarf,
    YellowDwarf,
    WhiteStar,
    BlueStar,

    RedGiant,
    OrangeGiant,
    YellowGiant,
	WhiteGiant,
    BlueGiant,

    RedSupergiant,
	OrangeSupergiant,
    YellowSupergiant,
    WhiteSupergiant,
    BlueSupergiant,

    WhiteDwarf,
    NeutronStar,
}

impl StarType {
    pub fn get_properties(&self) -> (StarColor, u32, u32, f32, f32) {
        match self {
            StarType::RedDwarf         => (StarColor::Red,    1_000,  3_000, 0.3, 0.6),
            StarType::OrangeDwarf      => (StarColor::Orange, 3_001,  5_000, 0.6, 0.9),
            StarType::YellowDwarf      => (StarColor::Yellow, 5_001,  7_000, 0.9, 1.2),
            StarType::WhiteStar        => (StarColor::White,  7_001, 10_000, 1.2, 2.0),
            StarType::BlueStar         => (StarColor::Blue,  10_001, 20_000, 2.0, 3.5),

            StarType::RedGiant         => (StarColor::Red,    2_500,  3_500, 3.5, 6.0),
            StarType::OrangeGiant      => (StarColor::Orange, 3_501,  5_000, 6.0, 10.0),
            StarType::YellowGiant      => (StarColor::Yellow, 5_001,  7_000, 6.0, 10.0),
            StarType::WhiteGiant       => (StarColor::White,  7_001, 10_000, 5.0, 8.5),
            StarType::BlueGiant        => (StarColor::Blue,  10_001, 30_000, 4.0, 8.0),

            StarType::RedSupergiant    => (StarColor::Red,    3_000,  4_500, 16.0, 25.0),
            StarType::OrangeSupergiant => (StarColor::Orange, 4_501,  5_500, 14.0, 22.0),
            StarType::YellowSupergiant => (StarColor::Yellow, 5_501,  7_000, 13.0, 20.0),
            StarType::WhiteSupergiant  => (StarColor::White,  7_001, 10_000, 11.0, 18.0),
            StarType::BlueSupergiant   => (StarColor::Blue,  10_001, 50_000,  9.0, 15.0),

            StarType::WhiteDwarf       => (StarColor::White,  8_000, 25_000, 0.02, 0.06),
            StarType::NeutronStar      => (StarColor::Blue,  30_000, 60_000, 0.004, 0.016),
        }
    }

	pub fn get_weight(&self) -> u32 {
        match self {
            StarType::RedDwarf => 730,
            StarType::OrangeDwarf => 140,
			StarType::YellowDwarf => 70,
			StarType::WhiteStar => 20,
			StarType::BlueStar => 5,
			StarType::RedGiant => 15,
			StarType::YellowGiant => 5,
			StarType::OrangeGiant => 3,
			StarType::WhiteGiant => 2,
			StarType::BlueGiant => 1,
			StarType::RedSupergiant => 2,
			StarType::OrangeSupergiant => 1,
			StarType::YellowSupergiant => 1,
			StarType::WhiteSupergiant => 1,
			StarType::BlueSupergiant => 1,
            StarType::WhiteDwarf => 2,
            StarType::NeutronStar => 1,
        }
    }

	pub fn draw_type(rng: &mut StdRng) -> StarType{
		let choices: Vec<StarType> = StarType::iter().collect();

		let weights: Vec<u32> = choices.iter().map(|star| star.get_weight()).collect(); 
		
		let dist = WeightedIndex::new(&weights).unwrap();
		choices[dist.sample(rng)]
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
}