use bevy::prelude::*;

use crate::GameState;
use systems::{dynamic_lens_exposure, dynamic_star_scaling};

// pub mod components;
// pub mod math;
pub mod systems;

pub struct VFXPlugin;

impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
			dynamic_lens_exposure,
			dynamic_star_scaling
		))
        ;
    }
}