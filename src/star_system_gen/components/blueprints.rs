use crate::star_system_gen::components::OrbitalBody;
pub struct SpaceBlueprint {
    pub orbit: f32,
    pub size: f32,
    pub gravity: u32,
    pub body_type: OrbitalBody,
    pub is_super: bool,
    pub n_of_moons: u32,
}