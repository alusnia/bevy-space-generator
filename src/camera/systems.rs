use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy::post_process::bloom::Bloom;
use bevy::camera::visibility::RenderLayers;
use bevy::render::view::Hdr;
use bevy::core_pipeline::tonemapping::Tonemapping;
use std::f32::consts::PI;

use crate::{GameRng, GameState};
use crate::camera::components::{OrbitalCamera, CameraFocusedOn, CameraView};
use crate::camera::math::{is_camera_active, interpolate_camera_movement, update_camera_transform};
use crate::universe_gen::components::UniverseStar;
use crate::star_system_gen::components::LocalStarSystem;

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

fn get_scroll_delta(scroll_evr: &mut MessageReader<MouseWheel>) -> f32 {
    let mut scroll_y_delta = 0.0;
    for ev in scroll_evr.read() {
        scroll_y_delta += ev.y;
    }
    scroll_y_delta
}

pub fn exit_star_system(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &CameraView, &mut OrbitalCamera)>,
    local_star_query: Query<Entity, With<LocalStarSystem>>,
    mut universe_star_query: Query<&mut Visibility, With<UniverseStar>>,
) {
    for (entity, view, mut orbit) in &mut camera_query {
        if *view == CameraView::StarSystem {
            commands.entity(entity).despawn();
        } else if *view == CameraView::Galaxy {

            orbit.target_radius = orbit.min_radius + 20.0;
            orbit.radius = orbit.target_radius;
        }
    }
    
    for entity in &local_star_query { commands.entity(entity).despawn(); }
    for mut visibility in &mut universe_star_query { *visibility = Visibility::Inherited; }
}