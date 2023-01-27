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
    pub forward_bias: f32,
    pub left_bias: f32,
    pub right_bias: f32,
    pub weight: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
}

impl Species {
    pub fn new(config: SpeciesConfig) -> GameResult<Species> {
        let species = Species { 
            sensor_size: config.sensor_size,
            sensor_angle: config.sensor_angle,
            sensor_distance: config.sensor_distance,
            turn_speed: config.turn_speed,
            move_speed: config.move_speed,
            forward_bias: config.forward_bias,
            left_bias: config.left_bias,
            right_bias: config.right_bias,
            weight: config.weight,
            color_r: config.color_r,
            color_g: config.color_g,
            color_b: config.color_b,
         };

        return Ok(species);
    }
}
