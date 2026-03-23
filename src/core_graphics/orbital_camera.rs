use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy::post_process::bloom::Bloom;
use bevy::camera::visibility::RenderLayers;
use std::f32::consts::PI;
use bevy::render::view::Hdr;
use bevy::core_pipeline::tonemapping::Tonemapping;

use crate::{GameRng, GameState, OrbitalCamera};
use crate::enums::CameraView;

pub fn spawn_orbital_camera(
    commands: &mut Commands, 
    view: CameraView,
    base_orbit: Option<&OrbitalCamera>
) {
    let (yaw, pitch, focus, target_focus) = base_orbit.map(|o| (o.yaw, o.pitch, o.focus, o.target_focus)).unwrap_or((0.0, 0.0, Vec3::ZERO, Vec3::ZERO));

    let (layer, order, clear_color, start_radius, min_rad, max_rad) = match view {
        CameraView::Galaxy => (
            RenderLayers::layer(0), 0, ClearColorConfig::Default, 
            50.0, 10.0, 200.0
        ),
        CameraView::StarSystem => (
            RenderLayers::layer(1), 1, ClearColorConfig::None, 
            549.0, 20.0, 550.0
        ),
    };

	let orbit = OrbitalCamera {
        focus: focus, 
		target_focus: target_focus,
        radius: start_radius,
		target_radius: start_radius,
        yaw, pitch,
        min_radius: min_rad,
		max_radius: max_rad,
    };

    commands.spawn((
        Camera3d::default(),
        Camera {
            order,
            clear_color,
            ..default()
        },
        Hdr,
        Bloom::default(),
        Tonemapping::TonyMcMapface,
        Transform::default(),
        layer,
		orbit,
        view,
    ));
}

pub fn orbital_camera_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll_evr: MessageReader<MouseWheel>,
    time: Res<Time>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&mut Transform, &mut OrbitalCamera, &CameraView)>, 
) {
    let delta = time.delta_secs();
    let state = current_state.get();
    let scroll_y_delta = get_scroll_delta(&mut scroll_evr);

    for (mut transform, mut orbit, view) in &mut query {
        
        handle_rotation_input(&keyboard, &mut orbit, delta);

        if is_camera_active(view, state) {
            if scroll_y_delta != 0.0 {
                
                let zoom_speed = if orbit.max_radius > 300.0 { 20.0 } else { 2.0 };
                
                orbit.target_radius -= scroll_y_delta * zoom_speed;
                orbit.target_radius = orbit.target_radius.clamp(orbit.min_radius, orbit.max_radius);
            }
            
            check_state_transitions(&orbit, view, state, &mut next_state);
        }

        interpolate_camera_movement(&mut orbit, delta);

        if orbit.is_changed() {
            update_camera_transform(&mut transform, &orbit);
        }
    }
}

fn handle_rotation_input(keyboard: &Res<ButtonInput<KeyCode>>, orbit: &mut OrbitalCamera, delta: f32) {
    let rotation_speed = 2.0 * delta;
    let mut d_yaw = 0.0;
    let mut d_pitch = 0.0;
    
    if keyboard.pressed(KeyCode::KeyA) { d_yaw -= rotation_speed; }
    if keyboard.pressed(KeyCode::KeyD) { d_yaw += rotation_speed; }
    if keyboard.pressed(KeyCode::KeyW) { d_pitch += rotation_speed; }
    if keyboard.pressed(KeyCode::KeyS) { d_pitch -= rotation_speed; }

    if d_yaw != 0.0 || d_pitch != 0.0 {
        orbit.yaw += d_yaw;
        orbit.pitch += d_pitch;
        orbit.pitch = orbit.pitch.clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
    }
}

fn handle_zoom_input(scroll_y_delta: f32, orbit: &mut OrbitalCamera) {
    orbit.target_radius -= scroll_y_delta * 1.5; 
    orbit.target_radius = orbit.target_radius.clamp(3.5, 50.0);
}

fn check_state_transitions(
    orbit: &OrbitalCamera, 
    view: &CameraView,
    current_state: &GameState, 
    next_state: &mut ResMut<NextState<GameState>>
) {
    if *view == CameraView::Galaxy && orbit.target_radius <= orbit.min_radius && *current_state == GameState::GalaxyMap {
        next_state.set(GameState::StarSystem);
    } 
    else if *view == CameraView::StarSystem && orbit.target_radius >= orbit.max_radius && *current_state == GameState::StarSystem {
        next_state.set(GameState::GalaxyMap);
    }
}
fn interpolate_camera_movement(orbit: &mut OrbitalCamera, delta: f32) {
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

fn update_camera_transform(transform: &mut Transform, orbit: &OrbitalCamera) {
    let x = orbit.radius * orbit.yaw.sin() * orbit.pitch.cos();
    let y = orbit.radius * orbit.pitch.sin();
    let z = orbit.radius * orbit.yaw.cos() * orbit.pitch.cos();

    transform.translation = orbit.focus + Vec3::new(x, y, z);
    *transform = transform.looking_at(orbit.focus, Vec3::Y);
}

fn get_scroll_delta(scroll_evr: &mut MessageReader<MouseWheel>) -> f32 {
    let mut scroll_y_delta = 0.0;
    for ev in scroll_evr.read() {
        scroll_y_delta += ev.y;
    }
    scroll_y_delta
}

fn is_camera_active(view: &CameraView, current_state: &GameState) -> bool {
    match (view, current_state) {
        (CameraView::Galaxy, GameState::GalaxyMap) => true,
        (CameraView::StarSystem, GameState::StarSystem) => true,
        _ => false,
    }
}