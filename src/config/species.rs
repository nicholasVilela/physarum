use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpeciesConfig {
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
