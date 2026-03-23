use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;

use crate::camera::components::{OrbitalCamera, CameraView, CameraFocusedOn};
use crate::universe_gen::components::UniverseStar;

pub fn dynamic_lens_exposure(
    mut query: Query<(&OrbitalCamera, &mut Bloom, &CameraView)>,
) {
    for (orbit, mut bloom, view) in &mut query {
        
        let range = (orbit.max_radius - orbit.min_radius).max(0.1); 
        let zoom_percent = ((orbit.radius - orbit.min_radius) / range).clamp(0.0, 1.0);

        match view {
            CameraView::Galaxy => {
                bloom.intensity = 0.1 + (0.5 * zoom_percent);
            },
            CameraView::StarSystem => {
               
                let docelowa_intensywnosc = 0.6 - (0.4 * zoom_percent); 
                
                bloom.intensity = docelowa_intensywnosc;
            }
        }
    }
}

pub fn dynamic_star_scaling(
    camera_query: Query<&Transform, With<OrbitalCamera>>,
    mut star_query: Query<(Entity, &mut Transform, &UniverseStar), Without<OrbitalCamera>>,
    selected_system: Res<CameraFocusedOn>, 
) {
    if let Ok(cam_transform) = camera_query.single() {
        let camera_pos = cam_transform.translation;

        for (entity, mut star_transform, star) in &mut star_query {
            let distance = camera_pos.distance(star_transform.translation);
            let multiplier: f32;

            if Some(entity) == selected_system.entity_id{
                multiplier = (distance / 5.0).clamp(1.0, 10.0);
            } else {
                multiplier = (distance / 50.0).clamp(1.0, 5.0);
            }
            
            star_transform.scale = Vec3::splat(star.scale * multiplier);
        }
    }
}
