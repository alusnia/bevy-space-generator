use bevy::prelude::*;
use strum_macros::EnumIter;

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

pub struct StarProperties {
    pub color: StarColor,
    pub min_temp: u32,
    pub max_temp: u32,
    pub min_radius: f32,
    pub max_radius: f32,
}

impl StarType {
    pub fn get_properties(&self) -> StarProperties {
        match self {
            StarType::RedDwarf         => StarProperties {color: StarColor::Red,    min_temp: 1_000,  max_temp: 3_000, min_radius: 0.3, max_radius: 0.6},
            StarType::OrangeDwarf      => StarProperties {color: StarColor::Orange, min_temp: 3_001,  max_temp: 5_000, min_radius: 0.6, max_radius: 0.9},
            StarType::YellowDwarf      => StarProperties {color: StarColor::Yellow, min_temp: 5_001,  max_temp: 7_000, min_radius: 0.9, max_radius: 1.2},
            StarType::WhiteStar        => StarProperties {color: StarColor::White,  min_temp: 7_001, max_temp: 10_000, min_radius: 1.2, max_radius: 2.0},
            StarType::BlueStar         => StarProperties {color: StarColor::Blue,  min_temp: 10_001, max_temp: 20_000, min_radius: 2.0, max_radius: 3.5},

            StarType::RedGiant         => StarProperties {color: StarColor::Red,    min_temp: 2_500,  max_temp: 3_500, min_radius: 3.5, max_radius: 6.0},
            StarType::OrangeGiant      => StarProperties {color: StarColor::Orange, min_temp: 3_501,  max_temp: 5_000, min_radius: 6.0, max_radius: 10.0},
            StarType::YellowGiant      => StarProperties {color: StarColor::Yellow, min_temp: 5_001,  max_temp: 7_000, min_radius: 6.0, max_radius: 10.0},
            StarType::WhiteGiant       => StarProperties {color: StarColor::White,  min_temp: 7_001, max_temp: 10_000, min_radius: 5.0, max_radius: 8.5},
            StarType::BlueGiant        => StarProperties {color: StarColor::Blue,  min_temp: 10_001, max_temp: 30_000, min_radius: 4.0, max_radius: 8.0},

            StarType::RedSupergiant    => StarProperties {color: StarColor::Red,    min_temp: 3_000,  max_temp: 4_500, min_radius: 16.0, max_radius: 25.0},
            StarType::OrangeSupergiant => StarProperties {color: StarColor::Orange, min_temp: 4_501,  max_temp: 5_500, min_radius: 14.0, max_radius: 22.0},
            StarType::YellowSupergiant => StarProperties {color: StarColor::Yellow, min_temp: 5_501,  max_temp: 7_000, min_radius: 13.0, max_radius: 20.0},
            StarType::WhiteSupergiant  => StarProperties {color: StarColor::White,  min_temp: 7_001, max_temp: 10_000, min_radius: 11.0, max_radius: 18.0},
            StarType::BlueSupergiant   => StarProperties {color: StarColor::Blue,  min_temp: 10_001, max_temp: 50_000, min_radius:  9.0, max_radius: 15.0},

            StarType::WhiteDwarf       => StarProperties {color: StarColor::White,  min_temp: 8_000, max_temp: 25_000, min_radius: 0.02, max_radius: 0.06},
            StarType::NeutronStar      => StarProperties {color: StarColor::Blue,  min_temp: 30_000, max_temp: 60_000, min_radius: 0.004, max_radius: 0.016},
        }
    }
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

pub struct Chunk {
	pub has_star: bool,
	pub position: (f32, f32, f32)
}