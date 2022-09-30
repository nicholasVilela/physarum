use ggez::{GameResult};
use crate::{SpeciesConfig};


#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
pub struct Species {
    pub sensor_size: f32,
    pub sensor_angle: f32,
    pub sensor_distance: f32,
    pub turn_speed: f32,
    pub move_speed: f32,
    pub random_forward_strength: f32,
    pub random_left_strength: f32,
    pub random_right_strength: f32,
    pub weight: f32,
    pub color: [f32; 3],
    pub color2: [f32; 3],
}

impl Species {
    pub fn new(config: SpeciesConfig) -> GameResult<Species> {
        let species = Species { 
            sensor_size: config.sensor_size,
            sensor_angle: config.sensor_angle,
            sensor_distance: config.sensor_distance,
            turn_speed: config.turn_speed,
            move_speed: config.move_speed,
            random_forward_strength: config.random_forward_strength,
            random_left_strength: config.random_left_strength,
            random_right_strength: config.random_right_strength,
            weight: config.weight,
            color: config.color,
            color2: config.color,
         };

        return Ok(species);
    }
}
