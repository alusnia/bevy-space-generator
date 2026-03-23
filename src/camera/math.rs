use bevy::prelude::*;
use crate::GameState;
use crate::camera::components::{OrbitalCamera, CameraView};

// fn handle_zoom_input(scroll_y_delta: f32, orbit: &mut OrbitalCamera) {
//     orbit.target_radius -= scroll_y_delta * 1.5; 
//     orbit.target_radius = orbit.target_radius.clamp(3.5, 50.0);
// }

pub fn interpolate_camera_movement(orbit: &mut OrbitalCamera, delta: f32) {
    let distance_to_target = orbit.focus.distance(orbit.target_focus);
    let radius_diff = (orbit.target_radius - orbit.radius).abs();

    if distance_to_target > 0.001 || radius_diff > 0.001 {
        let lerp_speed = 5.0 * delta;
        orbit.focus = orbit.focus.lerp(orbit.target_focus, lerp_speed);
        orbit.radius = orbit.radius + (orbit.target_radius - orbit.radius) * lerp_speed;
    } else if distance_to_target > 0.0 || radius_diff > 0.0 {
        orbit.focus = orbit.target_focus;
        orbit.radius = orbit.target_radius;
    }
}

pub fn update_camera_transform(transform: &mut Transform, orbit: &OrbitalCamera) {
    let x = orbit.radius * orbit.yaw.sin() * orbit.pitch.cos();
    let y = orbit.radius * orbit.pitch.sin();
    let z = orbit.radius * orbit.yaw.cos() * orbit.pitch.cos();

    transform.translation = orbit.focus + Vec3::new(x, y, z);
    *transform = transform.looking_at(orbit.focus, Vec3::Y);
}

pub fn is_camera_active(view: &CameraView, current_state: &GameState) -> bool {
    match (view, current_state) {
        (CameraView::Galaxy, GameState::GalaxyMap) => true,
        (CameraView::StarSystem, GameState::StarSystem) => true,
        _ => false,
    }
}