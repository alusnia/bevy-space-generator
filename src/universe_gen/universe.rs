use std::process::id;
use bevy::picking::pointer::Location;
use bevy::prelude::*;
use rand::{RngExt, SeedableRng};
use rand::distr::Alphanumeric;
use rand::rngs::StdRng;
use crate::enums::{CameraView, StarColor, StarType};
use crate::orbital_camera::spawn_orbital_camera;
use crate::{CameraFocusedOn, GameRng, GameState};
use crate::structs::{BelongsTo, Chunk, OrbitalCamera, Star, Universe, UniverseStar};

pub fn spawn_universe(
    mut commands: Commands,
    mut focused_on: ResMut<CameraFocusedOn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_rng: ResMut<GameRng>
) {
    let galaxy_id = commands.spawn(Universe {name: "Galaktyka".into() }).id();
    let mut rng = StdRng::seed_from_u64(game_rng.0.random());

    let base_mesh = meshes.add(Sphere::new(0.01));
    let hitbox_mesh = meshes.add(Sphere::new(0.1));
    let hitbox_material = materials.add(create_hitbox_material());
    let palette = create_star_palette(&mut materials);

    spawn_orbital_camera(&mut commands, CameraView::Galaxy, None);

    let start_pos: f32 = 2000.0;
    let end_pos: f32 = -2000.0; 
    let step: f32 = 50.0;

    let steps = ((start_pos - end_pos) / step) as i32;

    for x_step in 0..=steps {
        let x = start_pos - (x_step as f32 * step);
        for y_step in 0..=steps {
            let y = start_pos - (y_step as f32 * step);
            for z_step in 0..=steps {
                let z = start_pos - (z_step as f32 * step);
                let location = (x, y, z);
                
                let chunk = get_chunk(location, &mut rng);
                if chunk.has_star {
                    spawn_star(
                        &mut commands, 
                        &mut focused_on, 
                        base_mesh.clone(), 
                        &palette, 
                        hitbox_mesh.clone(), 
                        hitbox_material.clone(), 
                        &mut rng, 
                        chunk.position, 
                        galaxy_id
                    );
                }
            }
        }
    }
}

fn create_hitbox_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    }
}

fn create_star_palette(materials: &mut ResMut<Assets<StandardMaterial>>) -> [Handle<StandardMaterial>; 5] {
    [
        materials.add(StandardMaterial {
            emissive: StarColor::Red.to_linear_rgba() * 20.0,
            base_color: Color::BLACK,
            ..default()
        }),
        materials.add(StandardMaterial {
            emissive: StarColor::Orange.to_linear_rgba() * 20.0,
            base_color: Color::BLACK,
            ..default()
        }),
        materials.add(StandardMaterial {
            emissive: StarColor::Yellow.to_linear_rgba() * 30.0,
            base_color: Color::BLACK,
            ..default()
        }),
        materials.add(StandardMaterial {
            emissive: StarColor::White.to_linear_rgba() * 40.0,
            base_color: Color::BLACK,
            ..default()
        }),
        materials.add(StandardMaterial {
            emissive: StarColor::Blue.to_linear_rgba() * 50.0,
            base_color: Color::BLACK,
            ..default()
        }),
    ]
}

fn spawn_star(
    commands: &mut Commands,
    focused_on: &mut ResMut<CameraFocusedOn>,
    mesh_handle: Handle<Mesh>,
    palette: &[Handle<StandardMaterial>; 5],
    hitbox_mesh: Handle<Mesh>,
    hitbox_material: Handle<StandardMaterial>,
    mut rng: &mut StdRng,
    position: (f32, f32, f32),
    galaxy_id: Entity
) {
    let (x, y, z) = position;
	let kind = StarType::draw_type(rng);
    let (color, temp_min, temp_max, scale_min, scale_max) = StarType::get_properties(&kind);

    let material_handle = match color {
        StarColor::Red => palette[0].clone(),
        StarColor::Orange => palette[1].clone(),
        StarColor::Yellow => palette[2].clone(),
        StarColor::White => palette[3].clone(),
        StarColor::Blue => palette[4].clone(),
    };

	let scale = rng.random_range(scale_min..=scale_max);

	let hitbox_scale: f32 = match scale {
		..0.4 => 10.0,
		0.4..1.2 => 4.0,
		1.2..6.0 => 1.0,
		6.0..14.0 => 0.7,
		_ => 0.3,
	};

    let star_id = commands.spawn((
        UniverseStar{
            name: (&mut rng)
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect(),
            temperature: rng.random_range(temp_min..=temp_max),
            scale,
            color,
			kind,
            seed: rng.random()
            },
        BelongsTo(galaxy_id),
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
    ))
    .observe(select_star)
    .with_children(|parent| {
        parent.spawn((
            Mesh3d(hitbox_mesh),
            MeshMaterial3d(hitbox_material),
            Transform::from_scale(Vec3::splat(hitbox_scale)),
        ));
    })
    .id();

    if x.abs() <= 1.0 && y.abs() <= 1.0 && z.abs() <= 1.0 {
        focused_on.entity_id = Some(star_id);
    }
}

fn get_chunk(location: (f32, f32, f32), rng: &mut StdRng) -> Chunk {
    let mut chunk: Chunk = Chunk { has_star: false, position: (0.0, 0.0, 0.0) };
    let (x, y, z) = location;

    if location == (0.0,0.0,0.0) {
        chunk.has_star = true;
        chunk.position = (0.0, 0.0, 0.0); 
        return chunk
    }

    let max = x.abs().max(y.abs()).max(z.abs());

    if (rng.random_range(0.0..35.0) * 50.0) / (rng.random_range(1.0..1.5) * max) > 1.0 {
        chunk.has_star = true;
        chunk.position.0 = x + rng.random_range(-10.0..10.0);
        chunk.position.1 = y + rng.random_range(-10.0..10.0);
        chunk.position.2 = z + rng.random_range(-10.0..10.0);
    }
    chunk
}

fn select_star(
    trigger: On<Pointer<Click>>,
    mut focused_on: ResMut<CameraFocusedOn>,
    star_query: Query<(&UniverseStar, &Transform)>,
    mut camera_query: Query<&mut OrbitalCamera>,
	current_state: Res<State<GameState>>
) {
	if *current_state.get() == GameState::GalaxyMap {
		let selected_entity = trigger.entity;
		focused_on.entity_id = Some(selected_entity);

		if let Ok((_star, star_transform)) = star_query.get(selected_entity) {
			let star_position = star_transform.translation;

			for mut orbit in &mut camera_query {
				orbit.target_focus = star_position;
				orbit.target_radius = 15.0;
			}
		}
	}
}