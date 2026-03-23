use rand::{RngExt, rngs::StdRng};
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use strum::IntoEnumIterator;

use crate::universe_gen::components::{StarType, StarColor, Chunk};


pub fn get_weight(obj: &StarType) -> u32 {
    match obj {
        StarType::RedDwarf => 730,
        StarType::OrangeDwarf => 140,
		StarType::YellowDwarf => 70,
		StarType::WhiteStar => 20,
		StarType::BlueStar => 5,
		StarType::RedGiant => 15,
		StarType::YellowGiant => 5,
		StarType::OrangeGiant => 3,
		StarType::WhiteGiant => 2,
		StarType::BlueGiant => 1,
		StarType::RedSupergiant => 2,
		StarType::OrangeSupergiant => 1,
		StarType::YellowSupergiant => 1,
		StarType::WhiteSupergiant => 1,
		StarType::BlueSupergiant => 1,
        StarType::WhiteDwarf => 2,
        StarType::NeutronStar => 1,
    }
}

pub fn draw_type(rng: &mut StdRng) -> StarType{
	let choices: Vec<StarType> = StarType::iter().collect();
	let weights: Vec<u32> = choices.iter().map(|star: &StarType| get_weight(star)).collect(); 
	
	let dist = WeightedIndex::new(&weights).unwrap();
	choices[dist.sample(rng)]
}

pub fn get_chunk(location: (f32, f32, f32), rng: &mut StdRng) -> Chunk {
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