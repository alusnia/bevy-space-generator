use bevy::camera::visibility::{RenderLayers, calculate_bounds};
use bevy::ecs::spawn;
use bevy::color::LinearRgba;
use bevy::light::NotShadowCaster;
use bevy::math::ops::powf;
use bevy::prelude::*;
// use bevy::render::view::RenderLayers;
// use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::Color;
use bevy::ui::AvailableSpace;
use rand::{RngExt, SeedableRng};
use rand::distr::Alphanumeric;
use rand::rngs::StdRng;

use crate::spawn_orbit_ring;
use crate::{CameraFocusedOn};
use crate::structs::{BelongsTo, LocalStarSystem, OrbitalCamera, SpaceBlueprint, Star, UniverseStar};
use crate::enums::{CameraView, OrbitalBody, StarColor};
use crate::orbital_camera::spawn_orbital_camera;

use std::f32::consts::TAU;



