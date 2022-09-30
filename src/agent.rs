use std::{f32::consts::{TAU}};
use rand::{Rng};
use ggez::{GameResult};
use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Agent {
    pub position: [f32; 2],
    pub angle: f32,
    pub species: u32,
}

impl Agent {
    pub fn default() -> GameResult<Agent> {
        let position = [0.0, 0.0];
        let angle = 0.0;
        let species = 0;

        let agent = Agent {
            position,
            angle,
            species,
        };

        return Ok(agent);
    }

    pub fn random_angle<R: Rng + ?Sized>(mut self, rng: &mut R) -> GameResult<Agent> {
        let angle = rng.gen::<f32>() * TAU;

        self.angle = angle;

        return Ok(self);
    }

    pub fn random_position<R: Rng + ?Sized>(mut self, rng: &mut R) -> GameResult<Agent> {
        let position = [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)];

        self.position = position;
        
        return Ok(self);
    }

    pub fn random_species<R: Rng + ?Sized>(mut self, rng: &mut R, species_count: u32) -> GameResult<Agent> {
        let species = rng.gen_range(0..species_count) as u32;

        self.species = species;

        return Ok(self);
    }

    pub fn with_species(mut self, species: u32) -> GameResult<Agent> {
        self.species = species;

        return Ok(self);
    }
}
