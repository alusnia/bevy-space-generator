use bevy::prelude::*;

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

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum CameraView {
    Galaxy,
    StarSystem,
}

#[derive(Resource)]
pub struct CameraFocusedOn { pub entity_id: Option<Entity> }