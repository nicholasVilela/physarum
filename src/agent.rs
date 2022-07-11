use std::{f32::consts::{TAU}};
use rand::{Rng};
use ggez::{GameResult};
use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Agent {
    pub position: [f32; 2],
    pub angle: f32,
    pub seed: f32,
}

impl Agent {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> GameResult<Agent> {
        let angle = rng.gen::<f32>() * TAU;
        // let position = [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)];
        let position = [0.0, 0.0];
        let seed = rng.gen_range(-1000000.0..1000000.0);

        let agent = Agent {
            position,
            angle,
            seed,
        };

        return Ok(agent);
    }
}
